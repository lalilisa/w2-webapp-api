use axum::extract;
use tracing::error;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::common::utils;
use crate::model::req::auth::LoginRequest;
use crate::model::res::auth::LoginResponse;

pub async fn process(extract::Json(payload) : extract::Json<LoginRequest>) -> Result<ApiResponse<LoginResponse>, ApiError>{

    let result = crate::function::auth::login(payload).await;

    match result {
        Ok(res) => Ok(res.into()),
        Err(_e) => {
            error!("{}", _e);
            Err(utils::match_app_err(_e))
        },
    }
}



