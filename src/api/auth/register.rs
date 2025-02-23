use axum::extract;
use tracing::error;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::common::utils;
use crate::model::req::auth::RegisterRequest;
use crate::model::res::auth::RegisterResponse;

pub async fn process(extract::Json(payload) : extract::Json<RegisterRequest>) -> Result<ApiResponse<RegisterResponse>, ApiError>{

    let result = crate::function::auth::register(payload).await;

    match result {
        Ok(res) => Ok(res.into()),
        Err(_e) => {
            error!("{}", _e);
            Err(utils::match_app_err(_e))
        },
    }

}