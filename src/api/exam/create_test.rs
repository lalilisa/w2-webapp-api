use axum::Json;
use tracing::error;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::common::utils::match_app_err;
use crate::function;
use crate::model::req::exam::TestDataRequest;
use crate::model::res::exam::TestExamResponse;

pub async fn process(Json(payload) : Json<TestDataRequest>) -> Result<ApiResponse<TestExamResponse>, ApiError> {

    let result = function::exam::create_test(payload).await;

    match result {
        Ok(test) => Ok(test.into()),
        Err(_err) =>{
            error!("{:?}", _err);
            Err(match_app_err(_err))
        }
    }
}