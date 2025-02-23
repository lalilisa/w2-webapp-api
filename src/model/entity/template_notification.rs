use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_template::{DDLTemplate, DeleteTemplate, InsertTemplate, SelectTemplate, TableName, UpdateTemplate};

#[derive(
    Debug,
    Clone,
    FromRow,
    Default,
    TableName,
    InsertTemplate,
    SelectTemplate,
    UpdateTemplate,
    DeleteTemplate,
    DDLTemplate,
    Serialize,
    Deserialize,
)]

#[table_name = "template_notification"]
#[tp_select_one(by = "id, is_active")]
#[tp_select_page(by = "is_active",order = "created_at desc", fn_name = "find_by_is_active")]
#[tp_update(by = "id")]
#[tp_delete(by = "id")]
pub struct TemplateNotification {

    #[auto]
    pub id: i32,
    pub code: String,
    pub is_active: bool,
    pub title: String,
    pub content: String,
    pub template_type: Option<String>, // "type" is a reserved keyword, so we use "type_"
    #[auto]
    pub created_at: Option<DateTime<Utc>>, // Auto-generated timestamp
    pub updated_at: Option<DateTime<Utc>>, // Nullable update timestamp
}
