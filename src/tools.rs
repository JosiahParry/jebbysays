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
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TaskIdParams {
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddObjectiveParams {
    pub title: String,
    pub context: Option<String>,
    pub priority: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModifyObjectiveParams {
    pub id: String,
    pub title: Option<String>,
    pub context: Option<String>,
    pub priority: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddTaskParams {
    pub title: String,
    pub context: Option<String>,
    pub deadline: Option<Timestamp>,
    pub priority: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub objective: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModifyTaskParams {
    pub id: String,
    pub title: Option<String>,
    pub context: Option<String>,
    pub deadline: Option<Timestamp>,
    pub priority: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub objective: Option<String>,
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
        sqlx::migrate!().run(&db).await?;

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

    #[tool(description = "Create a new task")]
    async fn add_task(
        &self,
        Parameters(params): Parameters<AddTaskParams>,
    ) -> Result<CallToolResult, McpError> {
        let id = Ulid::new().to_string();
        let created = Timestamp::now().as_millisecond();
        let priority = params.priority.unwrap_or(3);
        let objective = params.objective.unwrap_or_else(|| "other".to_string());
        let tags = params.tags.map(Json);
        let deadline = params.deadline.map(|t| t.as_millisecond());

        sqlx::query(
            "INSERT INTO tasks (id, created, deadline, priority, title, context, tags, objective)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(created)
        .bind(deadline)
        .bind(priority)
        .bind(&params.title)
        .bind(&params.context)
        .bind(&tags)
        .bind(&objective)
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully added task with id: {id}"
        ))]))
    }

    #[tool(description = "Create a new objective")]
    async fn add_objective(
        &self,
        Parameters(params): Parameters<AddObjectiveParams>,
    ) -> Result<CallToolResult, McpError> {
        let id = Ulid::new().to_string();
        let priority = params.priority.unwrap_or(3);

        sqlx::query("INSERT INTO objectives (id, title, context, priority) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&params.title)
            .bind(&params.context)
            .bind(priority)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully added objective with id: {id}"
        ))]))
    }

    #[tool(description = "Delete an objective by ID")]
    async fn delete_objective(
        &self,
        Parameters(params): Parameters<ObjectiveIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let rows = sqlx::query("DELETE FROM objectives WHERE id = ?")
            .bind(&params.id)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .rows_affected();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Deleted {rows} objective"
        ))]))
    }

    #[tool(description = "Delete a task by ID")]
    async fn delete_task(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let rows = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(&params.id)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .rows_affected();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Deleted {rows} task"
        ))]))
    }

    #[tool(description = "Mark a task as completed")]
    async fn complete_task(
        &self,
        Parameters(params): Parameters<TaskIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let completed = Timestamp::now().as_millisecond();
        let rows = sqlx::query("UPDATE tasks SET completed = ? WHERE id = ?")
            .bind(completed)
            .bind(&params.id)
            .execute(&self.db)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
            .rows_affected();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Marked {rows} task as completed"
        ))]))
    }

    #[tool(description = "Modify fields of an existing task")]
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

        sqlx::query(
            "INSERT OR REPLACE INTO tasks (id, created, completed, deadline, priority, title, context, tags, objective)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully updated task {}",
            params.id
        ))]))
    }

    #[tool(description = "Modify fields of an existing objective")]
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
            "INSERT OR REPLACE INTO objectives (id, title, context, priority) VALUES (?, ?, ?, ?)",
        )
        .bind(&obj.id)
        .bind(&title)
        .bind(&context)
        .bind(priority)
        .execute(&self.db)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Successfully updated objective {}",
            params.id
        ))]))
    }
}
