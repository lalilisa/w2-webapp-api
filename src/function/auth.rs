use crate::common::error::{AppError, AppErrorCode};
use crate::common::jwt::create_jwt;
use crate::common::utils::{hash_text, verify_hash};
use crate::model::entity::user::Users;
use crate::repo;
use anyhow::Error;
use std::collections::HashMap;
use chrono::{Utc};
use tracing::{info};
use crate::model::req::auth::{LoginRequest, RegisterRequest};
use crate::model::res::auth::{LoginResponse, RegisterResponse};

pub async fn register(req: RegisterRequest) -> Result<RegisterResponse, AppError> {
    let exist_user_result = repo::user::find_by_username(&req.username).await;

    if let Err(_err) = exist_user_result {
        return Err(AppError::InternalError(Error::msg("INTERNAL_SEVER_ERROR")));
    }

    let exist_user = exist_user_result?;

    if let Some(_user) = exist_user {
        return Err(AppError::BadRequest {
            message: "Username is exists, please using other one".to_string(),
            backtrace: None,
            code: AppErrorCode::BadInput,
        });
    }

    let user = Users {
        id: 0,
        username: req.username,
        name: req.name,
        address: req.address,
        admin: false,
        email: req.email,
        password: hash_text(&req.password)?,
        phone: req.phone,
        last_auth_change: None,
        created_by: None,
        created_at: None,
        updated_by: None,
        updated_at: None,
        secret_otp: None,
    };
    repo::user::create_user(&user).await?;

    let register = RegisterResponse {
        status: String::from("SUCCESS"),
        message: String::from("Create Account SUCCESS"),
    };

    Ok(register)
}

pub async fn login(req: LoginRequest) -> Result<LoginResponse, AppError> {

    let user_result = repo::user::find_by_username(&req.username).await;

    if let Err(_err) = user_result {
        return Err(AppError::InternalError(Error::msg("INTERNAL_SEVER_ERROR")));
    }

    let user = user_result?;

    if let None = user {
        return Err(AppError::BadRequest {
            message: "Username is not exists".to_string(),
            backtrace: None,
            code: AppErrorCode::BadInput,
        });
    }

    let user = user.unwrap();
    let t = Utc::now();

    if !verify_hash(&user.password, &req.password) {
        return Err(AppError::BadRequest {
            message: "Password is wrong".to_string(),
            backtrace: None,
            code: AppErrorCode::BadInput,
        });
    }
    info!("query took {}", Utc::now().signed_duration_since(t).num_milliseconds());


    let mut claims = HashMap::new();

    claims.insert("username".to_string(), user.username.clone());
    claims.insert("user_id".to_string(), user.id.to_string());
    claims.insert("sub".to_string(), user.username.clone());
    let jwt_result = create_jwt(claims);

    if let Err(_err) = jwt_result {
        return Err(AppError::InternalError(Error::msg("INTERNAL_SEVER_ERROR")));
    }
    let jwt = jwt_result.ok();

    let login_response = LoginResponse {
        user_id: user.id,
        username: user.username,
        access_token: jwt,
        refresh_token: None,
    };

    Ok(login_response)
}
