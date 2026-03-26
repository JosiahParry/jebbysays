use crate::portfolio::Portfolio;
use jiff::Timestamp;
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    prompt_router, schemars, tool, tool_router,
};
use serde::Deserialize;
use sqlx::{SqlitePool, types::Json};
use std::path::PathBuf;
use ulid::Ulid;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ObjectiveIdParams {
    #[schemars(description = "ULID of the objective. Read objectives://all to get valid IDs.")]
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TaskIdParams {
    #[schemars(description = "ULID of the task. Prefer tasks://objective/{id}/incomplete or tasks://objective/{id}/completed; fall back to tasks://incomplete or tasks://completed if the objective is unknown.")]
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddObjectiveParams {
    #[schemars(description = "Short, descriptive title for the objective.")]
    pub title: String,
    #[schemars(description = "Additional context or detail about the objective.")]
    pub context: Option<String>,
    #[schemars(description = "Priority from 1 (highest) to 5 (lowest). Defaults to 3.")]
    pub priority: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModifyObjectiveParams {
    #[schemars(description = "ULID of the objective to modify. Read objectives://all to get valid IDs.")]
    pub id: String,
    #[schemars(description = "Updated title for the objective.")]
    pub title: Option<String>,
    #[schemars(description = "Updated context or detail about the objective.")]
    pub context: Option<String>,
    #[schemars(description = "Priority from 1 (highest) to 5 (lowest).")]
    pub priority: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddTaskParams {
    #[schemars(description = "Short, descriptive title for the task.")]
    pub title: String,
    #[schemars(description = "Additional context or detail about the task.")]
    pub context: Option<String>,
    #[schemars(description = "Optional deadline as a UTC timestamp.")]
    pub deadline: Option<Timestamp>,
    #[schemars(description = "Priority from 1 (highest) to 5 (lowest). Defaults to 3.")]
    pub priority: Option<i64>,
    #[schemars(description = "Optional list of tags for supplementary categorization.")]
    pub tags: Option<Vec<String>>,
    #[schemars(description = "ULID of the objective this task belongs to. Read objectives://all to get valid IDs.")]
    pub objective: String,
    #[schemars(
        description = "ULID of a task that must be completed before this one can start. Read tasks://incomplete or tasks://objective/{id}/incomplete to get valid IDs. Tasks that are depended upon by others should be given higher priority, as completing them unblocks downstream work."
    )]
    pub depends_on: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModifyTaskParams {
    #[schemars(description = "ULID of the task to modify. Prefer tasks://objective/{id}/incomplete or tasks://objective/{id}/completed; fall back to tasks://incomplete or tasks://completed if the objective is unknown.")]
    pub id: String,
    #[schemars(description = "Updated title for the task.")]
    pub title: Option<String>,
    #[schemars(description = "Updated context or detail about the task.")]
    pub context: Option<String>,
    #[schemars(description = "Updated deadline as a UTC timestamp.")]
    pub deadline: Option<Timestamp>,
    #[schemars(description = "Priority from 1 (highest) to 5 (lowest).")]
    pub priority: Option<i64>,
    #[schemars(description = "Updated list of tags for supplementary categorization.")]
    pub tags: Option<Vec<String>>,
    #[schemars(description = "ULID of the objective this task belongs to. Read objectives://all to get valid IDs.")]
    pub objective: Option<String>,
    #[schemars(
        description = "ULID of a task that must be completed before this one can start. Read tasks://incomplete or tasks://objective/{id}/incomplete to get valid IDs. Tasks that are depended upon by others should be given higher priority, as completing them unblocks downstream work."
    )]
    pub depends_on: Option<String>,
}

#[tool_router]
#[prompt_router]
impl Portfolio {
    pub async fn new(path: PathBuf) -> anyhow::Result<Self> {
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let url = format!("sqlite:{}?mode=rwc", path.display());
        let db = SqlitePool::connect(&url).await?;

        sqlx::migrate!("../migrations").run(&db).await?;

        Ok(Self {
            db,
            user_id: "local".to_string(),
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        })
    }

    pub fn with_user(db: SqlitePool, user_id: String) -> Self {
        Self {
            db,
            user_id,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(
        description = "Create a new task. The objective field is required and must be a valid objective ULID — read the objectives://all resource to get available objective IDs. Use depends_on to link a task to a prerequisite; tasks that are depended upon by others should be given higher priority, as completing them unblocks downstream work."
    )]
    async fn add_task(
        &self,
        Parameters(params): Parameters<AddTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let id = Ulid::new().to_string();
        let created = Timestamp::now().as_millisecond();
        let priority = params.priority.unwrap_or(3);
        let objective = params.objective;
        let tags = params.tags.map(Json);
        let deadline = params.deadline.map(|t| t.as_millisecond());

        sqlx::query(
            "INSERT INTO tasks (id, created, deadline, priority, title, context, tags, objective, user_id, depends_on)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(created)
        .bind(deadline)
        .bind(priority)
        .bind(&params.title)
        .bind(&params.context)
        .bind(&tags)
        .bind(&objective)
        .bind(&self.user_id)
        .bind(&params.depends_on)
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully added task with id: {id}"
        ))]))
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(description = "Create a new objective. Objectives are overarching goals you want to accomplish — tasks are tracked against them. Every task must be linked to an objective by its ULID.")]
    async fn add_objective(
        &self,
        Parameters(params): Parameters<AddObjectiveParams>,
    ) -> Result<CallToolResult, McpError> {
        let id = Ulid::new().to_string();
        let priority = params.priority.unwrap_or(3);

        sqlx::query(
            "INSERT INTO objectives (id, title, context, priority, user_id) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&params.title)
        .bind(&params.context)
        .bind(priority)
        .bind(&self.user_id)
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully added objective with id: {id}"
        ))]))
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(description = "Delete an objective by ID. Read objectives://all to get valid objective IDs.")]
    async fn delete_objective(
        &self,
        Parameters(params): Parameters<ObjectiveIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let rows = sqlx::query("DELETE FROM objectives WHERE id = ? AND user_id = ?")
            .bind(&params.id)
            .bind(&self.user_id)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .rows_affected();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Deleted {rows} objective"
        ))]))
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(description = "Delete an erroneously created task. Do not use this to complete a task — use complete_task instead. Prefer tasks://objective/{id}/incomplete to get valid task IDs; fall back to tasks://incomplete if the objective is unknown.")]
    async fn delete_task(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let rows = sqlx::query("DELETE FROM tasks WHERE id = ? AND user_id = ?")
            .bind(&params.id)
            .bind(&self.user_id)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .rows_affected();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Deleted {rows} task"
        ))]))
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(description = "Mark a task as completed. Prefer tasks://objective/{id}/incomplete to get valid task IDs; fall back to tasks://incomplete if the objective is unknown.")]
    async fn complete_task(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let completed = Timestamp::now().as_millisecond();
        let rows = sqlx::query("UPDATE tasks SET completed = ? WHERE id = ? AND user_id = ?")
            .bind(completed)
            .bind(&params.id)
            .bind(&self.user_id)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .rows_affected();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Marked {rows} task as completed"
        ))]))
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(
        description = "Modify fields of an existing task. Prefer tasks://objective/{id}/incomplete to get valid task IDs; fall back to tasks://incomplete if the objective is unknown. Use depends_on to link a task to a prerequisite; tasks that are depended upon by others should be given higher priority, as completing them unblocks downstream work."
    )]
    async fn modify_task(
        &self,
        Parameters(params): Parameters<ModifyTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let task = self
            .get_task(&params.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let title = params.title.unwrap_or(task.title);
        let context = params.context.or(task.context);
        let deadline = params
            .deadline
            .or(task.deadline)
            .map(|t| t.as_millisecond());
        let priority = params.priority.unwrap_or(task.priority);
        let tags = params.tags.or(task.tags).map(Json);
        let objective = params.objective.unwrap_or(task.objective);
        let depends_on = params.depends_on.or(task.depends_on);

        sqlx::query(
            "INSERT OR REPLACE INTO tasks (id, created, completed, deadline, priority, title, context, tags, objective, user_id, depends_on)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&task.id)
        .bind(task.created.as_millisecond())
        .bind(task.completed.map(|t| t.as_millisecond()))
        .bind(deadline)
        .bind(priority)
        .bind(&title)
        .bind(&context)
        .bind(&tags)
        .bind(&objective)
        .bind(&self.user_id)
        .bind(&depends_on)
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully updated task {}",
            params.id
        ))]))
    }

    #[tracing::instrument(skip(self), fields(user_id = %self.user_id))]
    #[tool(description = "Modify fields of an existing objective. Read objectives://all to get valid objective IDs.")]
    async fn modify_objective(
        &self,
        Parameters(params): Parameters<ModifyObjectiveParams>,
    ) -> Result<CallToolResult, McpError> {
        let obj = self
            .get_objective(&params.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let title = params.title.unwrap_or(obj.title);
        let context = params.context.or(obj.context);
        let priority = params.priority.unwrap_or(obj.priority);

        sqlx::query(
            "INSERT OR REPLACE INTO objectives (id, title, context, priority, user_id) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&obj.id)
        .bind(&title)
        .bind(&context)
        .bind(priority)
        .bind(&self.user_id)
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully updated objective {}",
            params.id
        ))]))
    }
}
