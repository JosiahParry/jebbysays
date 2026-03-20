use anyhow::Result;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::router::{prompt::PromptRouter, tool::ToolRouter},
    model::*,
    prompt_handler,
    service::RequestContext,
    tool_handler,
};
use sqlx::SqlitePool;

use crate::types::{Objective, Task, TaskRow};

#[derive(Clone, Debug)]
pub(crate) struct Portfolio {
    pub(crate) db: SqlitePool,
    pub(crate) user_id: String,
    pub(crate) tool_router: ToolRouter<Portfolio>,
    pub(crate) prompt_router: PromptRouter<Portfolio>,
}

impl Portfolio {
    pub async fn get_task(&self, id: &str) -> Result<Task> {
        let row = sqlx::query_as!(
            TaskRow,
            "SELECT id as \"id!\", created as \"created!\", completed, deadline, priority as \"priority!\", title as \"title!\", context, tags as \"tags: _\", objective as \"objective!\" FROM tasks WHERE id = ?",
            id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(row.into())
    }

    pub async fn list_all_tasks(&self) -> Result<Vec<Task>> {
        let rows = sqlx::query_as!(
            TaskRow,
            "SELECT id as \"id!\", created as \"created!\", completed, deadline, priority as \"priority!\", title as \"title!\", context, tags as \"tags: _\", objective as \"objective!\" FROM tasks"
        )
        .fetch_all(&self.db)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn list_incomplete_tasks(&self) -> Result<Vec<Task>> {
        let rows = sqlx::query_as!(
            TaskRow,
            "SELECT id as \"id!\", created as \"created!\", completed, deadline, priority as \"priority!\", title as \"title!\", context, tags as \"tags: _\", objective as \"objective!\" FROM tasks WHERE completed IS NULL"
        )
        .fetch_all(&self.db)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn list_completed_tasks(&self) -> Result<Vec<Task>> {
        let rows = sqlx::query_as!(
            TaskRow,
            "SELECT id as \"id!\", created as \"created!\", completed, deadline, priority as \"priority!\", title as \"title!\", context, tags as \"tags: _\", objective as \"objective!\" FROM tasks WHERE completed IS NOT NULL"
        )
        .fetch_all(&self.db)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn list_completed_tasks_since(&self, days: u32) -> Result<Vec<Task>> {
        let cutoff = jiff::Timestamp::now()
            .checked_sub(jiff::Span::new().days(days))
            .map_err(|e| anyhow::anyhow!(e))?
            .as_millisecond();
        let rows = sqlx::query_as!(
            TaskRow,
            "SELECT id as \"id!\", created as \"created!\", completed, deadline, priority as \"priority!\", title as \"title!\", context, tags as \"tags: _\", objective as \"objective!\" FROM tasks WHERE completed >= ?",
            cutoff
        )
        .fetch_all(&self.db)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_objective(&self, id: &str) -> Result<Objective> {
        let obj = sqlx::query_as!(
            Objective,
            "SELECT id as \"id!\", title as \"title!\", context, priority as \"priority!\" FROM objectives WHERE id = ?",
            id
        )
        .fetch_one(&self.db)
        .await?;
        Ok(obj)
    }

    pub async fn list_objectives(&self) -> Result<Vec<Objective>> {
        let objs = sqlx::query_as!(
            Objective,
            "SELECT id as \"id!\", title as \"title!\", context, priority as \"priority!\" FROM objectives"
        )
        .fetch_all(&self.db)
        .await?;
        Ok(objs)
    }
}

#[tool_handler]
#[prompt_handler]
impl ServerHandler for Portfolio {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_resources()
                .enable_tools()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_instructions(
            "Chief is your personal chief of staff. Manage tasks and objectives via tools and resources.".to_string()
        )
    }

    async fn list_resources(
        &self,
        request: Option<PaginatedRequestParams>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        self.handle_list_resources(request, ctx).await
    }

    async fn list_resource_templates(
        &self,
        request: Option<PaginatedRequestParams>,
        ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        self.handle_list_resource_templates(request, ctx).await
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        self.handle_read_resource(request, ctx).await
    }
}
