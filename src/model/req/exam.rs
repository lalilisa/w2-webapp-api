use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestDataRequest {
    pub name: String,
    pub year: String,
    pub total_parts: i32,
    pub questions: i32,
    pub is_active: bool,
    pub test_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionDataRequest {
    pub test_id: i32,
    pub part: String, // Part number (e.g., "Part 1", "Part 2")
    pub question_number: i32, // The question number within the exam
    pub paragraph: Option<String>, // Nullable paragraph (for Parts 3, 4, 6, 7)
    pub question: String, // The question text
    pub options: serde_json::Value, // JSON array for multiple-choice answers
    pub correct_answer: String, // Only values 'A', 'B', 'C', 'D'
    pub audio_url: Option<String>, // Nullable audio URL (for listening sections)
    pub image_url: Option<String>, // Nullable image URL (for Part 1)
    pub explanation: Option<String>, // Explanation of the correct answer
    pub test_section: String, // 'Listening' or 'Reading'
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryPartTestRequest {
    pub test_id: i32,
    pub part: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserTestRequest {
    pub test_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserTestSubmitRequest {
    pub id: i32,
    pub listening: HashMap<i32, String>,
    pub reading: HashMap<i32, String>,
}






