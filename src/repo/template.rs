use crate::common::response::page::{IntoPage, Page};
use crate::get_db_conn;
use crate::model::entity::template_notification::TemplateNotification;
use crate::model::Pagination;
use anyhow::Result;

pub async fn find_all_templates(
    pagination: Pagination,
) -> Result<Page<TemplateNotification>> {
    let data = TemplateNotification::find_by_is_active(&true, pagination, get_db_conn()?)
        .await?;

    let page = data.into_page(pagination);

    Ok(page)
}

pub async fn find_one(id: &i32) -> Result<Option<TemplateNotification>> {
    let conn = get_db_conn()?;
    let data = TemplateNotification::find_one_by_id_and_is_active(id, &true, conn).await?;
    Ok(data)
}

pub async fn create_template(entity: &TemplateNotification) -> Result<TemplateNotification> {
    let conn = get_db_conn()?;
    let test = TemplateNotification::insert_return(entity, conn).await?;
    Ok(test)
}

pub async fn update_template(id: &i32, entity: &TemplateNotification) -> Result<()> {
    let conn = get_db_conn()?;
    TemplateNotification::update_by_id(id, entity, conn).await?;
    Ok(())
}

pub async fn delete_template(id: &i32) -> Result<()> {
    let conn = get_db_conn()?;
    TemplateNotification::delete_by_id(id, conn).await?;
    Ok(())
}
