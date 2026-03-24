use rmcp::{ErrorData as McpError, RoleServer, model::*, service::RequestContext};
use serde_json::json;

use crate::portfolio::Portfolio;

impl Portfolio {
    #[tracing::instrument(skip_all, fields(user_id = %self.user_id))]
    pub(crate) async fn handle_list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                RawResource::new("tasks://all", "All tasks. Every task must be linked to an objective via its objective ID.").no_annotation(),
                RawResource::new("tasks://incomplete", "Incomplete tasks. Each task is assigned to an objective by its ULID — tags are supplementary.").no_annotation(),
                RawResource::new("tasks://completed", "Completed tasks. Tasks are organized by objective ID — tags are supplementary metadata.").no_annotation(),
                RawResource::new("objectives://all", "All objectives. Objectives are the top-level goals — tasks must reference an objective by its ULID.").no_annotation(),
            ],
            next_cursor: None,
            meta: None,
        })
    }

    #[tracing::instrument(skip_all, fields(user_id = %self.user_id))]
    pub(crate) async fn handle_list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            resource_templates: vec![
                RawResourceTemplate::new("task://{id}", "A single task by its ULID. Tasks always have an objective field containing the parent objective's ULID.").no_annotation(),
                RawResourceTemplate::new("objective://{id}", "A single objective by its ULID. Use this ID when linking tasks to an objective.").no_annotation(),
                RawResourceTemplate::new(
                    "tasks://completed/{days}",
                    "Tasks completed in the last N days.",
                )
                .no_annotation(),
                RawResourceTemplate::new("tasks://objective/{id}/all", "All tasks for a specific objective. Provide the objective's ULID as {id}.").no_annotation(),
                RawResourceTemplate::new("tasks://objective/{id}/incomplete", "Incomplete tasks for a specific objective. Provide the objective's ULID as {id}.").no_annotation(),
                RawResourceTemplate::new("tasks://objective/{id}/completed", "Completed tasks for a specific objective. Provide the objective's ULID as {id}.").no_annotation(),
            ],
            next_cursor: None,
            meta: None,
        })
    }

    #[tracing::instrument(skip_all, fields(user_id = %self.user_id, uri = %request.uri))]
    pub(crate) async fn handle_read_resource(
        &self,
        request: ReadResourceRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let uri = request.uri.as_str();

        let content = match uri {
            "tasks://all" => {
                let tasks = self
                    .list_all_tasks()
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            "tasks://incomplete" => {
                let tasks = self
                    .list_incomplete_tasks()
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            "tasks://completed" => {
                let tasks = self
                    .list_completed_tasks()
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            "objectives://all" => {
                let objs = self
                    .list_objectives()
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&objs).unwrap()
            }
            uri if uri.starts_with("tasks://completed/") => {
                let days = uri["tasks://completed/".len()..]
                    .parse::<u32>()
                    .map_err(|_| {
                        McpError::invalid_params("days must be a positive integer", None)
                    })?;
                let tasks = self
                    .list_completed_tasks_since(days)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            uri if uri.starts_with("task://") => {
                let id = &uri["task://".len()..];
                let task = self
                    .get_task(id)
                    .await
                    .map_err(|e| McpError::resource_not_found(e.to_string(), None))?;
                serde_json::to_string(&task).unwrap()
            }
            uri if uri.starts_with("tasks://objective/") && uri.ends_with("/all") => {
                let id = &uri["tasks://objective/".len()..uri.len() - "/all".len()];
                let tasks = self
                    .list_tasks_by_objective(id)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            uri if uri.starts_with("tasks://objective/") && uri.ends_with("/incomplete") => {
                let id = &uri["tasks://objective/".len()..uri.len() - "/incomplete".len()];
                let tasks = self
                    .list_incomplete_tasks_by_objective(id)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            uri if uri.starts_with("tasks://objective/") && uri.ends_with("/completed") => {
                let id = &uri["tasks://objective/".len()..uri.len() - "/completed".len()];
                let tasks = self
                    .list_completed_tasks_by_objective(id)
                    .await
                    .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                serde_json::to_string(&tasks).unwrap()
            }
            uri if uri.starts_with("objective://") => {
                let id = &uri["objective://".len()..];
                let obj = self
                    .get_objective(id)
                    .await
                    .map_err(|e| McpError::resource_not_found(e.to_string(), None))?;
                serde_json::to_string(&obj).unwrap()
            }
            _ => {
                return Err(McpError::resource_not_found(
                    "unknown resource",
                    Some(json!({"uri": uri})),
                ));
            }
        };

        Ok(ReadResourceResult::new(vec![ResourceContents::text(
            content,
            request.uri,
        )]))
    }
}
