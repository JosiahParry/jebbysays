use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
    prompt,
    schemars::JsonSchema,
};
use serde::Deserialize;

use crate::portfolio::Portfolio;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BriefingArgs {
    #[schemars(description = "Objective name to filter by (optional)")]
    pub objective: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RetroArgs {
    #[schemars(description = "Number of days to look back (default 7)")]
    pub days: Option<u32>,
}

impl Portfolio {
    #[prompt(
        name = "briefing",
        description = "Morning portfolio: outstanding tasks grouped by objective, optionally filtered to one objective"
    )]
    async fn briefing(
        &self,
        Parameters(args): Parameters<BriefingArgs>,
    ) -> Result<GetPromptResult, McpError> {
        let objective_instruction = match args.objective {
            Some(name) => format!(
                "Focus only on the objective matching \"{name}\". Use the `objectives://all` resource to resolve the name to an ID, then filter tasks to that objective."
            ),
            None => "Include all objectives.".to_string(),
        };

        let messages = vec![PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Give me my morning briefing. Read `tasks://incomplete` and `objectives://all`. \
                 Group incomplete tasks by objective, ordered by priority (highest first). \
                 For each objective show its priority and list its tasks with deadlines if set. \
                 If a task has a depends_on field, check whether that dependency is still incomplete. \
                 Tasks with incomplete dependencies should be marked as blocked. \
                 Tasks that other tasks depend on should be treated as higher priority, as completing them unblocks downstream work. \
                 Be concise. {objective_instruction}"
            ),
        )];

        Ok(GetPromptResult::new(messages)
            .with_description("Morning portfolio briefing grouped by objective"))
    }

    #[prompt(
        name = "triage",
        description = "Help prioritize and shift focus across urgent tasks"
    )]
    async fn triage(&self) -> GetPromptResult {
        let messages = vec![PromptMessage::new_text(
            PromptMessageRole::User,
            "Help me triage my tasks. Read `tasks://incomplete` and `objectives://all`. \
             Identify what is most urgent based on priority and deadlines. \
             If a task has a depends_on field, check whether that dependency is still incomplete; if so, the task is blocked and should not be suggested as actionable. \
             Tasks that other tasks depend on should be treated as higher priority, as completing them unblocks downstream work. \
             Suggest what to focus on now, what can wait, flag anything overdue, and list any blocked tasks separately. \
             Be direct and actionable.",
        )];

        GetPromptResult::new(messages).with_description("Triage incomplete tasks by urgency")
    }

    #[prompt(
        name = "retro",
        description = "Retrospective: review completed work and remaining tasks over a time window"
    )]
    async fn retro(&self, Parameters(args): Parameters<RetroArgs>) -> GetPromptResult {
        let days = args.days.unwrap_or(7);
        let messages = vec![PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Run a retrospective for the last {days} days. \
                 Read `tasks://completed/{days}` for recently completed tasks and `tasks://incomplete` and `objectives://all`. \
                 Summarize what was accomplished, what is still outstanding, and whether my priorities \
                 are aligned with my objectives. Suggest any adjustments."
            ),
        )];

        GetPromptResult::new(messages)
            .with_description(format!("Retrospective for the last {days} days"))
    }
}
