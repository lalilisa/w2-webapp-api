use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
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
#[table_name = "t_otp"]
#[tp_select_one(by = "id")]
#[tp_select_one(by = "phone_number")]
#[tp_update(by = "id")]
pub struct OTP {
    #[auto]
    #[column(primary, type = "serial")]
    pub id: i32,
    pub phone_number: String, // Phone number to which the OTP is sent
    pub authen_key: String, // Unique key for the OTP request
    pub otp_code: String, // OTP code (Consider hashing this for security)
    pub expires_at: DateTime<Utc>, // Expiration time for the OTP
    pub is_verified: bool, // Has the OTP been verified?
    pub created_at: DateTime<Utc>, // Auto-generated timestamp
    pub updated_at: Option<DateTime<Utc>>, // Nullable update timestamp
}
