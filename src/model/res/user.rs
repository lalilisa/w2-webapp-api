use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::entity::user::Users;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoResponse {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub last_auth_change: Option<DateTime<Utc>>,
}

impl UserInfoResponse {
    pub fn from(user: Users) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            phone: user.phone,
            address: user.address,
            last_auth_change: user.last_auth_change,
        }
    }
}