use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse, MessageResponse};
use crate::function;
use crate::model::req::exam::TestDataRequest;
use axum::extract::Path;
use axum::Json;

pub async fn process(
    Path(test_id): Path<i32>,
    Json(payload): Json<TestDataRequest>,
) -> Result<ApiResponse<MessageResponse>, ApiError> {
    let result = function::exam::update_test(&test_id, payload).await;


     let res =check_response_api::<MessageResponse>(result);
     res
}
