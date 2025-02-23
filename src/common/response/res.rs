use std::fmt::{Display, Formatter};
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ApiResponse<T: Serialize> {
    pub message: Option<String>,
    pub data: Option<T>,
    pub code: i32,
    pub status: Option<StatusCode>,
}

#[derive(Debug, Serialize)]
pub struct ApiBody<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            message: None,
            data: Some(data),
            status: None,
            code: 0,
        }
    }

    pub fn error_msg(msg: impl Into<String>) -> Self {
        Self {
            message: Some(msg.into()),
            data: None,
            status: Some(StatusCode::INTERNAL_SERVER_ERROR),
            code: -1,
        }
    }

    pub fn error_msg_status(msg: impl Into<String>, status: impl Into<StatusCode>) -> Self {
        let status = status.into();
        if status.is_success() {
            Self {
                message: Some(msg.into()),
                data: None,
                status: Some(status.into()),
                code: 0,
            }
        } else {
            Self {
                message: Some(msg.into()),
                data: None,
                status: Some(status.into()),
                code: -1,
            }
        }
    }

    pub fn error_status(status: impl Into<StatusCode>) -> Self {
        let status = status.into();
        if status.is_success() {
            Self {
                message: None,
                data: None,
                status: Some(status.into()),
                code: 0,
            }
        } else {
            Self {
                message: None,
                data: None,
                status: Some(status.into()),
                code: -1,
            }
        }
    }

    pub fn error_msg_status_code(
        msg: impl Into<String>,
        status: impl Into<StatusCode>,
        code: impl Into<i32>,
    ) -> Self {
        Self {
            message: Some(msg.into()),
            data: None,
            status: Some(status.into()),
            code: code.into(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        if self.status.is_none() {
            (StatusCode::OK, Json(self.data)).into_response()
        } else {
            let body = ApiBody {
                message: self.message,
                code: self.code,
                data: self.data,
            };
            (self.status.unwrap(), Json(body)).into_response()
        }
    }
}

pub type ApiSuccessResult = Result<ApiResponse<()>, ApiResponse<()>>;
pub type ApiResult<T> = Result<ApiResponse<T>, ApiResponse<()>>;
pub type ApiErrorResult<E> = Result<ApiResponse<()>, ApiResponse<E>>;
pub type NoDataResponse = ApiResponse<()>;

impl<T: Serialize> From<T> for ApiResponse<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct MessageResponse {
    message: String,
}

impl MessageResponse {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

#[derive(Serialize, Error, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub status_code: u16,
    pub errors: Vec<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Err {} ", &self.status_code))
    }
}

impl ApiError {
    pub fn new(status_code: u16, err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code,
            errors,
        }
    }

    pub fn new_internal(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors,
        }
    }
    pub fn new_bad_request(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            errors,
        }
    }

    pub fn new_unauthorized(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            errors,
        }
    }

    pub fn new_not_found(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            errors,
        }
    }

    pub fn new_conflict(err: String) -> Self {
        let errors = vec![err];
        ApiError {
            status_code: StatusCode::CONFLICT.as_u16(),
            errors,
        }
    }

    pub fn append_error(&mut self, err: String) {
        self.errors.push(err);
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}


impl IntoResponse for MessageResponse {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(200).unwrap(), Json(self)).into_response()
    }
}