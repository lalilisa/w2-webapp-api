use crate::get_db_conn;
use crate::model::entity::user::Users;

pub async fn create_user(user_entity: &Users) -> anyhow::Result<()> {
    Users::insert(user_entity, get_db_conn()?).await?;
    Ok(())
}

pub async fn find_by_username(username: &str) -> anyhow::Result<Option<Users>> {
    let user = Users::find_one_by_username(username, get_db_conn()?).await?;
    Ok(user)
}

pub async fn find_all() -> anyhow::Result<Vec<Users>> {
    let user = Users::find_all(get_db_conn()?).await?;
    Ok(user)
}
