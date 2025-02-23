

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx_template::{delete, select, DDLTemplate, DeleteTemplate, InsertTemplate, SelectTemplate, TableName, UpdateTemplate};
use crate::model::entity::question::Questions;

#[derive(
    Debug,
    Clone,
    sqlx::FromRow,
    Default,
    TableName,
    InsertTemplate,
    SelectTemplate,
    UpdateTemplate,
    DeleteTemplate,
    DDLTemplate,
    Serialize,
    Deserialize,
)]
#[table_name = "user_test"]
#[tp_select_one(by = "id")]
#[tp_select_page(by = "user_id",order = "started_at desc")]
#[tp_update(by = "id")]
#[tp_delete(by = "id")]
pub struct UserTest {
    #[auto]
    pub id: i32,
    pub user_id: i32,
    pub test_id: i32,
    pub status: String,
    pub score: Option<i32>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    #[auto]
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}


#[select("select tp.* from user_test ut inner join t_tests t on ut.test_id = t.id inner join t_test_parts tp on t.id = tp.test_id where ut.id = :id and ut.user_id= :user_id")]
pub async fn find_question_by_user_test(id: &i32, user_id: &i32) -> Vec<Questions> {}