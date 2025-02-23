use axum::Json;
use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse};
use crate::function;
use crate::model::req::template_notification::TemplateNotificationRequest;
use crate::model::res::template_notification::TemplateResponse;

pub async fn process(Json(payload):Json<TemplateNotificationRequest>) -> Result<ApiResponse<TemplateResponse>,ApiError> {

    let result = function::template_notification::create_template(payload).await;

    let res =check_response_api(result);
    res
}