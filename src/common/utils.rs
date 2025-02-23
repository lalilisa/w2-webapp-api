use std::collections::HashMap;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;
use serde_json::Value;
use tracing::error;
use crate::common::error::AppError;
use crate::common::response::res::ApiError;

pub fn hash_text(plain_text: &str) -> anyhow::Result<String> {
    let hash_text = hash(plain_text, DEFAULT_COST)?;
    Ok(hash_text)
}

pub fn verify_hash(hash_txt: &str, plain_text: &str) -> bool {

    let verify =  verify(plain_text, hash_txt);

    if let Ok(_) = verify {
       return  true
    }
    false
}


pub fn match_app_err(app_error: AppError) -> ApiError {
    error!("{}", app_error);
    match app_error {
        AppError::ResourceNotFound { message, .. } => ApiError::new(404,message.to_string()),
        AppError::Unauthorized { message: _message, .. } => ApiError::new(401, "UNAUTHORIZED".to_string()),
        AppError::BadRequest { message, code: _code, .. } => ApiError::new(400, message.to_string()),
        AppError::InternalError(_) => ApiError::new(500,"INTERNAL_SERVER_ERROR".to_string()),
        AppError::Forbidden { backtrace: _ } => ApiError::new(403,"FORBIDDEN".to_string()),
    }
}

pub(crate) fn struct_to_hashmap<T>(obj: &T) -> HashMap<String, Value>
where
    T: Serialize,
{
    match serde_json::to_value(obj) {
        Ok(Value::Object(map)) => map.into_iter().collect(),
        _ => HashMap::new(),
    }
}