use rmcp::{ErrorData as McpError, RoleServer, model::*, service::RequestContext};
use serde_json::json;

use crate::portfolio::Portfolio;

impl Portfolio {
    pub(crate) async fn handle_list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                RawResource::new("tasks://all", "All tasks").no_annotation(),
                RawResource::new("tasks://incomplete", "Incomplete tasks").no_annotation(),
                RawResource::new("tasks://completed", "Completed tasks").no_annotation(),
                RawResource::new("objectives://all", "All objectives").no_annotation(),
            ],
            next_cursor: None,
            meta: None,
        })
    }

    pub(crate) async fn handle_list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            resource_templates: vec![
                RawResourceTemplate::new("task://{id}", "Task by ID").no_annotation(),
                RawResourceTemplate::new("objective://{id}", "Objective by ID").no_annotation(),
                RawResourceTemplate::new(
                    "tasks://completed/{days}",
                    "Tasks completed in the last N days",
                )
                .no_annotation(),
            ],
            next_cursor: None,
            meta: None,
        })
    }

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
