
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_template::{delete, update, DDLTemplate, DeleteTemplate, InsertTemplate, SelectTemplate, UpdateTemplate};

#[derive(Debug, FromRow, Serialize, Deserialize, Clone, InsertTemplate, SelectTemplate, UpdateTemplate,DeleteTemplate, DDLTemplate)]

#[table_name = "notification"]
#[tp_update(by = "id")]
#[tp_delete(by = "id")]
#[tp_select_page(by = "user_id",order = "created_at desc", fn_name = "find_by_user_id")]
pub struct Notification {

    #[auto]
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub status: String,   // "pending", "sent", "failed"
    pub sent_at: Option<DateTime<Utc>>,
    pub is_read: bool,
    #[auto]
    pub created_at: Option<DateTime<Utc>>, // Auto-generated timestamp
    pub updated_at: Option<DateTime<Utc>>, // Nullable update timestamp
}

#[delete("delete from notification WHERE id = ANY(:ids)")]
pub async fn delete_id_in(ids: &[i32]) {}

#[update("update notification set is_read = true WHERE id = ANY(:ids)")]
pub async fn read_id_in(ids: &[i32]) {}