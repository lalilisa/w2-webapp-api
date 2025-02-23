use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx_template::{DDLTemplate, DeleteTemplate, InsertTemplate, SelectTemplate, UpdateTemplate};

#[derive(
    Clone,
    Debug,
    sqlx::FromRow,
    Serialize,
    Deserialize,
    DeleteTemplate,
    UpdateTemplate,
    DDLTemplate,
    SelectTemplate,
    InsertTemplate,
)]
#[table_name = "t_users"]
#[tp_select_one(by = "id")]
#[tp_select_one(by = "username")]
#[tp_update(by = "id")]
pub struct Users {
    #[auto]
    #[column(primary, type = "serial")]
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub admin: bool,
    pub address: String,
    pub secret_otp: Option<String>,
    pub created_by: Option<String>,
    #[auto]
    pub created_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_auth_change: Option<DateTime<Utc>>,
}


