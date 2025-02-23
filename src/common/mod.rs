use crate::common::error::AppError;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::common::utils::match_app_err;
use serde::{ Serialize};

pub mod error;
pub mod utils;
pub mod jwt;
pub mod response;
pub mod kafka;

pub fn check_response_api<T>(result: Result<T, AppError>) -> Result<ApiResponse<T>,ApiError>
where T: From<T> + Serialize
{
    match result {
        Ok(v) => Ok(v.into()),
        Err(_err) => Err(match_app_err(_err))
    }
}

// pub fn check_response_api<T>(result: Result<Json<T>, AppError>) -> Result<T,ApiError>
// where T: From<T>
// {
//     match result {
//         Ok(v) => Ok(v.into()),
//         Err(_err) => Err(match_app_err(_err))
//     }
// }