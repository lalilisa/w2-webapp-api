use axum::extract::Query;
use axum::Json;
use tracing::log::error;
use crate::common::response::page::Page;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::common::utils::match_app_err;
use crate::function;
use crate::model::Pagination;
use crate::model::res::template_notification::TemplateResponse;
use crate::model::res::exam::TestExamResponse;

pub async fn process(Query(page) : Query<Pagination>) -> Result<Json<Page<TemplateResponse>>, ApiError> {

    let result = function::template_notification::get_templates(page).await;

    match result {
        Ok(page) => Ok(Json(page)),
        Err(_err) => {
            error!("{}", _err);
            Err(match_app_err(_err))
        }
    }
}