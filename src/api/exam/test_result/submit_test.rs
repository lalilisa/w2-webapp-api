use crate::common::check_response_api;
use crate::common::error::AppError;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::function;
use crate::middleware::UserAuthentication;
use crate::model::req::exam::{UserTestRequest, UserTestSubmitRequest};
use crate::model::res::exam::UserTestResponse;
use axum::{Extension, Json};

pub async fn process(
    Extension(current_user): Extension<UserAuthentication>,
    Json(payload): Json<UserTestSubmitRequest>,
) -> Result<ApiResponse<UserTestResponse>, ApiError> {
    let result = function::exam::submit_user_test(
        &current_user.username,
        &current_user.get_user_id(),
        payload,
    )
    .await;

    let res = check_response_api(result);

    res
}
