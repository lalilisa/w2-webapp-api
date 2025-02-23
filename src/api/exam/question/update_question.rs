use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::function;
use crate::model::entity::question::Questions;
use crate::model::req::exam::QuestionDataRequest;
use axum::Json;

pub async fn process(
    Json(payload): Json<QuestionDataRequest>,
) -> Result<ApiResponse<Option<Questions>>, ApiError> {
    let result = function::exam::update_question_test(payload).await;
    let res =check_response_api::<>(result);
    res
}