use crate::common::check_response_api;
use crate::common::response::res::{ApiError, ApiResponse, MessageResponse};
use crate::function;
use crate::model::req::template_notification::TemplateNotificationRequest;
use axum::extract::Path;

pub async fn process(
    Path(i32): Path<i32>,
    axum::Json(payload): axum::Json<TemplateNotificationRequest>,
) -> Result<ApiResponse<MessageResponse>, ApiError> {
    let result = function::template_notification::update_template(&i32,payload).await;

    let res = check_response_api(result);
    res
}
