use crate::common::error::{AppError, AppErrorCode};
use crate::common::kafka::producer::KafkaProducer;
use crate::common::kafka::KafkaMessageData;
use crate::common::response::page::Page;
use crate::common::response::res::MessageResponse;
use crate::model::entity::question::Questions;
use crate::model::entity::tests::Tests;
use crate::model::entity::user_test::UserTest;
use crate::model::req::exam::{
    QuestionDataRequest, TestDataRequest, UserTestRequest, UserTestSubmitRequest,
};
use crate::model::req::notification::SendNotification;
use crate::model::res::exam::{TestExamResponse, UserTestResponse};
use crate::model::Pagination;
use crate::repo;
use chrono::Utc;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use crate::common::utils::struct_to_hashmap;

pub async fn get_list_test(
    page: Pagination,
    test_type: &str,
) -> Result<Page<TestExamResponse>, AppError> {
    let page = repo::exam::find_english_test(page, test_type).await?;

    let page_res = page.map(|item| TestExamResponse::from(item));
    Ok(page_res)
}

pub async fn get_detail_test(id: &i32) -> Result<TestExamResponse, AppError> {
    let test_detail = repo::exam::find_one(id).await?;

    let res = test_detail
        .map(|test| TestExamResponse::from(test))
        .unwrap();

    Ok(res)
}

pub async fn create_test(req: TestDataRequest) -> Result<TestExamResponse, AppError> {
    let test = Tests {
        id: 0,
        test_type: "TOEIC".to_string(),
        name: req.name,
        year: req.year,
        is_active: true,
        total_parts: req.total_parts,
        questions: req.questions,
        created_at: None,
        updated_at: None,
        updated_by: None,
        created_by: None,
    };

    let test_entity = repo::exam::create_test(&test).await?;

    let res = TestExamResponse::from(test_entity);
    Ok(res)
}

pub async fn update_test(id: &i32, req: TestDataRequest) -> Result<MessageResponse, AppError> {
    let test_result = repo::exam::find_one(id).await?;

    if let None = test_result {
        return Err(AppError::BadRequest {
            message: "test is not exist".to_string(),
            code: AppErrorCode::ObjectNotFound,
            backtrace: None,
        });
    }

    let mut test = test_result.unwrap();

    test.test_type = req.test_type;
    test.name = req.name;
    test.year = req.year;
    test.questions = req.questions;
    test.total_parts = req.total_parts;
    test.is_active = req.is_active;

    repo::exam::update_test(id, &test).await?;
    Ok(MessageResponse::new("Update test successfully"))
}

pub async fn delete_test(id: &i32) -> Result<MessageResponse, AppError> {
    repo::exam::delete_test(id).await?;
    Ok(MessageResponse::new("Delete test successfully"))
}

pub async fn create_question_test(req: QuestionDataRequest) -> Result<Questions, AppError> {
    let exist_question_number = repo::exam::find_question_by_test_id_and_question_number(
        &req.test_id,
        &req.question_number,
        &req.test_section,
    )
    .await?;

    if let Some(_) = exist_question_number {
        return Err(AppError::BadRequest {
            message: "Question is  exist".to_string(),
            code: AppErrorCode::ObjectNotFound,
            backtrace: None,
        });
    }

    let question = Questions::from(req);

    let question = repo::exam::create_questions_test(&question).await?;

    Ok(question)
}

pub async fn update_question_test(req: QuestionDataRequest) -> Result<Option<Questions>, AppError> {
    let exist_question_number = repo::exam::find_question_by_test_id_and_question_number(
        &req.test_id,
        &req.question_number,
        &req.test_section,
    )
    .await?;

    if let None = exist_question_number {
        return Err(AppError::BadRequest {
            message: "Question is not exist".to_string(),
            code: AppErrorCode::ObjectNotFound,
            backtrace: None,
        });
    }

    let question_id = exist_question_number.unwrap().id;
    let question = Questions::from(req);

    let question = repo::exam::update_question_test(&question_id, &question).await?;

    Ok(question)
}

pub async fn delete_question(id: &i32) -> Result<MessageResponse, AppError> {
    repo::exam::delete_question(id).await?;
    Ok(MessageResponse::new("Delete test successfully"))
}

pub async fn get_part_test(test_id: &i32, part: &str) -> Result<Vec<Questions>, AppError> {
    let data = repo::exam::get_question_test(test_id, part).await?;
    Ok(data)
}

pub async fn create_user_test(
    user_id: &i32,
    req: UserTestRequest,
) -> Result<UserTestResponse, AppError> {
    let user_test = UserTest {
        id: 0,
        user_id: user_id.clone(),
        test_id: req.test_id,
        status: "UNCOMPLETED".to_string(),
        score: None,
        started_at: Some(Utc::now()),
        completed_at: None,
        created_at: None,
        updated_at: None,
    };

    let user_test = repo::exam::create_user_test(&user_test).await?;

    Ok(UserTestResponse::from(user_test))
}

pub async fn submit_user_test(
    username: &str,
    user_id: &i32,
    req: UserTestSubmitRequest,
) -> Result<UserTestResponse, AppError> {
    let user_test = repo::exam::find_one_user_test(&req.id).await?;

    if let None = user_test {
        return Err(AppError::ResourceNotFound {
            backtrace: None,
            message: "User test does not exist".to_string(),
        });
    }

    let questions = repo::exam::find_questions_user_test(&req.id, user_id).await?;

    let mut question_map: HashMap<i32, (String, String)> = HashMap::new();
    for question in &questions {
        question_map.insert(
            question.question_number,
            (
                question.correct_answer.clone(),
                question.test_section.clone(),
            ),
        );
    }

    let mut correct_listening = 0;
    let mut correct_reading = 0;

    for (question_number, user_answer) in &req.listening {
        if let Some((correct_answer, section)) = question_map.get(question_number) {
            if section == "Listening" && user_answer.to_uppercase() == correct_answer.to_uppercase()
            {
                correct_listening += 1;
            }
        }
    }

    for (question_number, user_answer) in &req.reading {
        if let Some((correct_answer, section)) = question_map.get(question_number) {
            if section == "Reading" && user_answer.to_uppercase() == correct_answer.to_uppercase() {
                correct_reading += 1;
            }
        }
    }

    let listening_score = correct_listening * 5;
    let reading_score = correct_reading * 5;
    let total_score = listening_score + reading_score;

    let mut user_test = user_test.unwrap();

    user_test.completed_at = Some(Utc::now());
    user_test.status = "COMPLETED".to_string();
    user_test.score = Some(total_score);

    let user_test = repo::exam::update_user_test(&req.id, &user_test).await?;

    let mut params = HashMap::new();

    params.insert("username".to_string(), Value::from(username.to_string()));

    let payload = SendNotification {
        template_code: "ABC123".to_string(),
        user_id: user_id.clone(),
        params,
    };

    let kafka_message_data = KafkaMessageData {
        action: "PUSH_NOTIFICATION".to_string(),
        payload: Some(struct_to_hashmap(&payload)),
    };
    let kafka_urls = env::var("KAFKA_URLS").expect("KAFKA_URL NOT FOUND");

    let mut producer =KafkaProducer::new(&kafka_urls);
    producer.send_message("notification",&kafka_message_data);
    Ok(UserTestResponse::from(user_test.unwrap()))
}

