use chrono::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub edited_at: Option<DateTime<Local>>,
}

impl Note {
    pub fn new(title: String) -> Self {
        Note {
            title,
            created_at: Local::now(),
            ..Default::default()
        }
    }
}
