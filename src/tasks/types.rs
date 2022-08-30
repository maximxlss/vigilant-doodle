use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Task {
    pub text: String,
    pub is_done: bool,
    pub created_at: DateTime<Local>,
    pub done_at: Option<DateTime<Local>>
}

impl Task {
    pub fn new(text: String) -> Self {
        Task {
            text,
            created_at: Local::now(),
            ..Default::default()
        }
    }
}
