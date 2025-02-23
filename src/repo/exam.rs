use crate::common::response::page::{IntoPage, Page};
use crate::get_db_conn;
use crate::model::entity::question::Questions;
use crate::model::entity::tests::Tests;
use crate::model::entity::user_test::UserTest;
use crate::model::entity::{question, tests, user_test};
use crate::model::Pagination;

pub async fn find_english_test(
    pagination: Pagination,
    test_type: &str,
) -> anyhow::Result<Page<Tests>> {
    let data = Tests::find_by_is_active_and_test_type(&true, test_type, pagination, get_db_conn()?)
        .await?;

    let page = data.into_page(pagination);

    Ok(page)
}

pub async fn find_one(id: &i32) -> anyhow::Result<Option<Tests>> {
    let data = Tests::find_one_by_id_and_is_active(id, &true, get_db_conn()?).await?;
    Ok(data)
}

pub async fn create_test(entity: &Tests) -> anyhow::Result<Tests> {
    let test = Tests::insert_return(entity, get_db_conn()?).await?;
    Ok(test)
}

pub async fn update_test(id: &i32, entity: &Tests) -> anyhow::Result<()> {
    Tests::update_by_id(id, entity, get_db_conn()?).await?;
    Ok(())
}

pub async fn delete_test(id: &i32) -> anyhow::Result<()> {
    Tests::delete_by_id(id, get_db_conn()?).await?;

    Ok(())
}

pub async fn delete_multi_test(id: &[i32]) -> anyhow::Result<()> {
    tests::delete_id_in(id, get_db_conn()?).await?;
    Ok(())
}

pub async fn get_question_test(test_id: &i32, part: &str) -> anyhow::Result<Vec<Questions>> {
    let data = Questions::find_by_test_id_and_part(part, test_id, get_db_conn()?).await?;
    Ok(data)
}

pub async fn delete_part_test(test_id: &i32, part: &str) -> anyhow::Result<()> {
    Questions::delete_by_part_and_test_id(part, test_id, get_db_conn()?).await?;
    Ok(())
}

pub async fn delete_question(id: &i32) -> anyhow::Result<()> {
    Questions::delete_by_id(id, get_db_conn()?).await?;
    Ok(())
}
pub async fn delete_question_in(id: &[i32]) -> anyhow::Result<()> {
    question::delete_id_in(id, get_db_conn()?).await?;
    Ok(())
}

pub async fn update_question_test(
    question_id: &i32,
    question: &Questions,
) -> anyhow::Result<Option<Questions>> {
    Questions::update_by_id_and_return(question_id, question, get_db_conn()?).await? as i32;
    let question = Questions::find_one_by_id(question_id, get_db_conn()?).await?;
    Ok(question)
}

pub async fn create_questions_test(question: &Questions) -> anyhow::Result<Questions> {
    let question = Questions::insert_return(question, get_db_conn()?).await?;
    Ok(question)
}

pub async fn find_question_by_test_id_and_question_number(
    test_id: &i32,
    question_number: &i32,
    test_section: &str,
) -> anyhow::Result<Option<Questions>> {
    let data = Questions::find_one_by_question_number_and_test_id_and_test_section(
        question_number,
        test_id,
        test_section,
        get_db_conn()?,
    )
    .await?;
    Ok(data)
}

pub async fn create_user_test(user_test: &UserTest) -> anyhow::Result<UserTest> {
    let data = UserTest::insert_return(user_test, get_db_conn()?).await?;
    Ok(data)
}

pub async fn find_one_user_test(id: &i32) -> anyhow::Result<Option<UserTest>> {
    let data = UserTest::find_one_by_id(id, get_db_conn()?).await?;
    Ok(data)
}
pub async fn update_user_test(id: &i32, user_test: &UserTest) -> anyhow::Result<Option<UserTest>> {
    UserTest::update_by_id(id, user_test, get_db_conn()?).await?;
    let data = UserTest::find_one_by_id(id, get_db_conn()?).await?;
    Ok(data)
}

pub async fn find_questions_user_test(user_test_id: &i32, user_id: &i32) -> anyhow::Result<Vec<Questions>> {
   let questions = user_test::find_question_by_user_test(user_test_id, user_id,get_db_conn()?).await?;
    Ok(questions)
}