use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub status: String,
    pub sent_at: Option<String>,
    pub is_read: bool,
    pub created_at: Option<NaiveDateTime>,
}
