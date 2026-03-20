use jiff::Timestamp;
use serde::Serialize;
use sqlx::{FromRow, types::Json};

#[derive(Debug, FromRow)]
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
}

#[derive(Debug, Serialize)]
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
}

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
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct Objective {
    pub id: String,
    pub title: String,
    pub context: Option<String>,
    pub priority: i64,
    pub user_id: String,
}
