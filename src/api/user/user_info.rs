use axum::Extension;
use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::function;
use crate::middleware::UserAuthentication;
use crate::model::res::user::UserInfoResponse;

pub async fn process(Extension(current_user): Extension<UserAuthentication>) -> Result<ApiResponse<Option<UserInfoResponse>>, ApiError> {
    let result = function::user::get_user_info(&current_user.username).await;
    let res = check_response_api(result);
    res
}
