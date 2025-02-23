use crate::model::entity::tests::Tests;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use chrono::{DateTime, Utc};
use crate::model::entity::user_test::UserTest;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestExamResponse {
    pub id: u32,
    pub name: String,
    pub total_parts: i32,
    pub questions: i32,
    pub test_type: String,
    pub year: String,
}

impl TestExamResponse {
    pub fn from(test: Tests) -> Self {
        Self {
            id: test.id as u32,
            name: test.name,
            total_parts: test.total_parts,
            questions: test.questions,
            test_type: test.test_type,
            year: test.year,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTestResponse {
    pub id: i32,
    pub status: String,
    pub score: Option<i32>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>
}

impl UserTestResponse {
    pub fn from(user_test: UserTest) -> Self {
        Self{
            id: user_test.id,
            status: user_test.status,
            score: user_test.score,
            started_at: user_test.started_at,
            completed_at: user_test.completed_at,
        }
    }
}