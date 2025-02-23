use crate::common::error::{AppError, AppErrorCode};
use crate::common::response::page::Page;
use crate::common::response::res::MessageResponse;
use crate::model::entity::template_notification::TemplateNotification;
use crate::model::req::template_notification::TemplateNotificationRequest;
use crate::model::res::template_notification::TemplateResponse;
use crate::model::Pagination;
use crate::repo;

pub async fn get_templates(page: Pagination) -> Result<Page<TemplateResponse>, AppError> {
    let page = repo::template::find_all_templates(page).await?;

    let page_res = page.map(|item| TemplateResponse::from(item));
    Ok(page_res)
}

pub async fn get_detail_test(id: &i32) -> Result<TemplateResponse, AppError> {
    let test_detail = repo::template::find_one(id).await?;

    let res = test_detail
        .map(|test| TemplateResponse::from(test))
        .unwrap();

    Ok(res)
}

pub async fn create_template(
    req: TemplateNotificationRequest,
) -> Result<TemplateResponse, AppError> {
    let template = TemplateNotification {
        id: 0,
        code: req.code,
        is_active: req.is_active,
        title: req.title,
        content: req.content,
        template_type: req.template_type,
        created_at: None,
        updated_at: None,
    };

    let template_entity = repo::template::create_template(&template).await?;
    repo::redis::set_hash("TEMPLATES", &template.code, &template_entity).await?;
    let res = TemplateResponse::from(template_entity);
    Ok(res)
}

pub async fn update_template(
    id: &i32,
    req: TemplateNotificationRequest,
) -> Result<MessageResponse, AppError> {
    let template_result = repo::template::find_one(id).await?;

    if let None = template_result {
        return Err(AppError::BadRequest {
            message: "update template is not exist".to_string(),
            code: AppErrorCode::ObjectNotFound,
            backtrace: None,
        });
    }

    let mut template = template_result.unwrap();

    template.template_type = req.template_type;
    template.code = req.code;
    template.title = req.title;
    template.content = req.content;
    template.is_active = req.is_active;

    repo::template::update_template(id, &template).await?;
    repo::redis::set_hash("TEMPLATES", &template.code, &template).await?;

    Ok(MessageResponse::new("Update test successfully"))
}

pub async fn delete_template(id: &i32) -> Result<MessageResponse, AppError> {
    let template = repo::template::find_one(id).await?;
    if let None = template {
        return Err(AppError::ResourceNotFound {
            backtrace: None,
            message: "delete template is not exist".to_string(),
        });
    }

    let template = template.unwrap();

    repo::redis::delete_hash("TEMPLATES", &template.code).await?;

    repo::template::delete_template(id).await?;
    Ok(MessageResponse::new("Delete test successfully"))
}
