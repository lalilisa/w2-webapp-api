use axum::extract::Query;
use axum::Json;
use tracing::log::error;
use crate::common::response::page::Page;
use crate::common::response::res::ApiError;
use crate::common::utils::match_app_err;
use crate::function;
use crate::model::Pagination;
use crate::model::res::exam::TestExamResponse;

pub async fn process(Query(page) : Query<Pagination>) -> Result<Json<Page<TestExamResponse>>, ApiError> {

    let result = function::exam::get_list_test(page,"TOEIC").await;

    match result {
        Ok(page) => Ok(Json(page)),
        Err(_err) => {
            error!("{}", _err);
            Err(match_app_err(_err))
        }
    }
}