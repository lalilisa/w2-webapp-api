use crate::common::response::page::{IntoPage, Page};
use crate::get_db_conn;
use crate::model::entity::notification;
use crate::model::entity::notification::Notification;
use crate::model::Pagination;
use anyhow::Result;

pub async fn find_all_templates(user_id : &i32,
    pagination: Pagination,
) -> Result<Page<Notification>> {
    let data = Notification::find_by_user_id(user_id, pagination, get_db_conn()?)
        .await?;

    let page = data.into_page(pagination);

    Ok(page)
}


pub async fn create_notification(entity: Notification) -> Result<()> {
    let conn = get_db_conn()?;
    Notification::insert(&entity, conn).await?;
    Ok(())
}

pub async fn read_notification(id: &[i32]) -> Result<()> {
    let conn = get_db_conn()?;
    notification::read_id_in(&id, conn).await?;
    Ok(())
}


pub async fn delete_notification(id: &[i32]) -> Result<()> {
    let conn = get_db_conn()?;
    notification::delete_id_in(&id, conn).await?;
    Ok(())
}
