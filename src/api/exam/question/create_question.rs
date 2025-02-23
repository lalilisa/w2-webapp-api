use crate::common::response::res::{ApiError, ApiResponse};
use crate::common::utils::match_app_err;
use crate::function;
use crate::model::entity::question::Questions;
use crate::model::req::exam::{QuestionDataRequest, TestDataRequest};
use axum::Json;
use tracing::error;

pub async fn process(
    Json(payload): Json<QuestionDataRequest>,
) -> Result<ApiResponse<Questions>, ApiError> {
    let result = function::exam::create_question_test(payload).await;

    match result {
        Ok(test) => Ok(test.into()),
        Err(_err) => {
            error!("{:?}", _err);
            Err(match_app_err(_err))
        }
    }
}
