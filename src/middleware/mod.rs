use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod authorization;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAuthentication {
    user_id: String,
    pub username: String,
}

impl UserAuthentication {

    pub fn get_user_id(&self) -> i32 {
        self.user_id.parse::<i32>().unwrap()
    }
}
