use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTPLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub name : String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendOTPRequest {
    pub phone_number: String,
}
