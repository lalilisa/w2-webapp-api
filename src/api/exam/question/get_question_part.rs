use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::function;
use crate::model::entity::question::Questions;
use axum::extract::Query;
use crate::model::req::exam::QueryPartTestRequest;

pub async fn process(
    Query(query): Query<QueryPartTestRequest>,
) -> Result<ApiResponse<Vec<Questions>>, ApiError> {
    let result = function::exam::get_part_test(&query.test_id, &query.part).await;

    let res = check_response_api(result);

    res
}
