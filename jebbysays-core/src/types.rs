use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::types::Json;

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
pub struct TaskRow {
    pub id: String,
    pub created: i64,
    pub completed: Option<i64>,
    pub deadline: Option<i64>,
    pub priority: i64,
    pub title: String,
    pub context: Option<String>,
    pub tags: Option<Json<Vec<String>>>,
    pub objective: String,
    pub user_id: String,
    pub depends_on: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub created: Timestamp,
    pub completed: Option<Timestamp>,
    pub deadline: Option<Timestamp>,
    pub priority: i64,
    pub title: String,
    pub context: Option<String>,
    pub tags: Option<Vec<String>>,
    pub objective: String,
    pub user_id: String,
    pub depends_on: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<TaskRow> for Task {
    fn from(row: TaskRow) -> Self {
        let tags = row.tags.map(|t| t.0);
        Task {
            id: row.id,
            created: Timestamp::from_millisecond(row.created).unwrap(),
            completed: row
                .completed
                .map(|ms| Timestamp::from_millisecond(ms).unwrap()),
            deadline: row
                .deadline
                .map(|ms| Timestamp::from_millisecond(ms).unwrap()),
            priority: row.priority,
            title: row.title,
            context: row.context,
            tags,
            objective: row.objective,
            user_id: row.user_id,
            depends_on: row.depends_on,
        }
    }
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: String,
    pub title: String,
    pub context: Option<String>,
    pub priority: i64,
    pub user_id: String,
}
