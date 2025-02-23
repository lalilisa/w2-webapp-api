use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx_template::{delete, DDLTemplate, DeleteTemplate, InsertTemplate, SelectTemplate, TableName, UpdateTemplate};

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
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
#[table_name = "t_tests"]
#[tp_select_one(by = "id, is_active")]
#[tp_select_page(by = "is_active, test_type",order = "year desc", fn_name = "find_by_is_active_and_test_type")]
#[tp_update(by = "id")]
#[tp_delete(by = "id")]
pub struct Tests {
    #[auto]
    #[column(primary, type = "serial")]
    pub id: i32,
    pub name: String,
    pub year: String,
    pub total_parts: i32,
    pub questions: i32,
    pub is_active: bool,
    pub test_type: String,
    pub created_by: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}


#[delete("delete from t_tests WHERE id = ANY(:ids)")]
pub async fn delete_id_in(ids: &[i32]) {}
