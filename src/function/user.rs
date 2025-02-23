use tracing::error;
use crate::common::error::{AppError, AppErrorCode};
use crate::model::entity::user::Users;
use crate::model::res::user::UserInfoResponse;
use crate::repo;

pub async fn get_user_info(username: &str) -> Result<Option<UserInfoResponse>, AppError> {
    let user = repo::user::find_by_username(username).await?;

    if user.is_none() {
        error!("No user found with username {}", username);
        return Err(AppError::Unauthorized {
            message: "Unauthorized".to_string(),
            backtrace: None,
        });
    }


    let user_info_res = user.map(|e| UserInfoResponse::from(e));

    Ok(user_info_res)
}