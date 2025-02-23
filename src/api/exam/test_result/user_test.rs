use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::function;
use crate::middleware::UserAuthentication;
use crate::model::req::exam::UserTestRequest;
use crate::model::res::exam::UserTestResponse;
use axum::{Extension, Json};

pub async fn process(Extension(current_user): Extension<UserAuthentication>,Json(payload):Json<UserTestRequest>) ->Result<ApiResponse<UserTestResponse>,ApiError>{

    let result = function::exam::create_user_test(&current_user.get_user_id(),payload).await;

    let res = check_response_api(result);

    res
}