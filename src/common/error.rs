

use thiserror::Error;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum AppErrorCode {
    Ok = 0,



    Unknown = -1,
    BadInput = -2,
    ObjectNotFound = -3,
    ValueNotMatch = -4,

    OtpSessionExpired = -12,
    TokenExpired = -13,
    ActionNotSetupYet = -14,
    ActionAlreadyDone = -15,
    ExceedActionLimit = -16,
    DuplicateOrg = -2556,

    InvalidResource = -5211,
    UnauthorizedResource = -5210,
}



#[derive(Error, Debug)]
pub enum AppError {
    #[error("{message}")]
    ResourceNotFound{message: String, backtrace: Option<String>},
    #[error("{message}")]
    Unauthorized{message: String, backtrace: Option<String>},
    #[error("{message}")]
    BadRequest{message: String, code: AppErrorCode, backtrace: Option<String>},
    #[error("Insufficient permissions")]
    Forbidden{backtrace: Option<String>},
    #[error(transparent)]
    InternalError(anyhow::Error),
}

impl AppError {

    // pub fn backtrace<F>(&self, func: F) -> Cow<'_, str>
    // where F: FnOnce(&anyhow::Error) -> String {
    //     fn to_cow(optional_string: Option<&String>) -> Cow<'_, str> {
    //         match optional_string {
    //             Some(s) => Cow::Borrowed(s.as_str()),
    //             None => Cow::Borrowed(""),
    //         }
    //     }
    //     match &self {
    //         AppError::ResourceNotFound { backtrace, .. } => to_cow(backtrace.as_ref()),
    //         AppError::Unauthorized { backtrace, .. } => to_cow(backtrace.as_ref()),
    //         AppError::BadRequest { backtrace, .. } => to_cow(backtrace.as_ref()),
    //         AppError::InternalError(e) => func(&e).into(),
    //         AppError::Forbidden { backtrace } => to_cow(backtrace.as_ref()),
    //     }
    // }
    //
    // pub fn resource_not_found(message: impl Into<String>) -> Self {
    //     // let need_backtrace = cc::get_enable_backtrace().ok().unwrap_or(&false);
    //     // let backtrace = if *need_backtrace {
    //     //     Some(utils::pretty_err(format!("{:?}", Backtrace::capture())))
    //     // } else {
    //     //     None
    //     // };
    //     Self::ResourceNotFound { message: message.into(), backtrace }
    // }
    // pub fn forbidden() -> Self {
    //     // let need_backtrace = cc::get_enable_backtrace().ok().unwrap_or(&false);
    //     // let backtrace = if *need_backtrace {
    //     //     Some(utils::pretty_err(format!("{:?}", Backtrace::capture())))
    //     // } else {
    //     //     None
    //     // };
    //     Self::Forbidden { backtrace }
    // }

    // pub fn unauthorized(message: impl Into<String>) -> Self {
    //     Self::Unauthorized { message: message.into(), backtrace }
    // }
    // pub fn bad_request(message: impl Into<String>) -> Self {
    //     Self::BadRequest { message: message.into(), backtrace: None , code: AppErrorCode::Unknown}
    // }
    // pub fn bad_request_with_code(message: impl Into<String>, code: AppErrorCode) -> Self {
    //     Self::BadRequest { message: message.into(), backtrace: None , code}
    // }
    //
    // pub fn bad_code(code: AppErrorCode) -> Self {
    //     Self::BadRequest { message: "".into(), backtrace: None , code}
    // }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError (err)
    }
}

// impl From<AppError> for NoDataResponse {
//     fn from(value: AppError) -> Self {
//         match value {
//             AppError::ResourceNotFound { message, .. } => NoDataResponse::error_msg_status(message, StatusCode::NOT_FOUND),
//             AppError::Unauthorized { message, .. } => NoDataResponse::error_msg_status(message, StatusCode::UNAUTHORIZED),
//             AppError::BadRequest { message, code, .. } => NoDataResponse::error_msg_status_code(message, StatusCode::BAD_REQUEST, code as i32),
//             AppError::InternalError(_) => NoDataResponse::status( StatusCode::INTERNAL_SERVER_ERROR),
//             AppError::Forbidden { backtrace: _ } => NoDataResponse::status( StatusCode::FORBIDDEN),
//         }
//     }
// }