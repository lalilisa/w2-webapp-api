use axum::extract::Path;
use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse, MessageResponse};
use crate::function;

pub async fn process(Path(test_id):Path<i32>) -> Result<ApiResponse<MessageResponse>,ApiError> {

    let result =function::exam::delete_test(&test_id).await;

    let res = check_response_api(result);

    res
}