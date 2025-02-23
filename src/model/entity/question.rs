use crate::model::req::exam::QuestionDataRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_template::{delete, DeleteTemplate, InsertTemplate, SelectTemplate, TableName, UpdateTemplate};

#[derive(
    Debug,
    Clone,
    FromRow,
    Default,
    TableName,
    InsertTemplate,
    SelectTemplate,
    UpdateTemplate,
    DeleteTemplate,
    Serialize,
    // DDLTemplate,
    Deserialize,
)]
#[table_name = "t_test_parts"]
#[tp_select_one(by = "id")]
#[tp_select_one(by = "test_id, question_number, test_section")]
#[tp_select_all(by = "test_id, question_number")]
#[tp_select_all(by = "test_id, part",order = "question_number asc", fn_name = "find_by_test_id_and_part")]
#[tp_update(by = "id", returning_id = 1, fn_name = "update_by_id_and_return")]
#[tp_delete(by = "id")]
#[tp_delete(by = "test_id, part")]
pub struct Questions {
    #[auto]
    // #[column(primary, type = "serial")]
    pub id: i32,
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
    #[auto]
    pub created_at: Option<DateTime<Utc>>, // Auto-generated timestamp
    pub updated_at: Option<DateTime<Utc>>, // Nullable update timestamp

}


impl Questions {

    pub fn from(req : QuestionDataRequest) -> Self {

        Self {
            id: 0,
            test_id: req.test_id,
            part: req.part,
            question_number: req.question_number,
            paragraph: req.paragraph,
            question: req.question,
            options: req.options,
            correct_answer: req.correct_answer,
            audio_url: req.audio_url,
            image_url: req.image_url,
            explanation: req.explanation,
            test_section: req.test_section,
            // created_by: None,
            created_at: None,
            // updated_by: None,
            updated_at: None,
        }
    }
}

#[delete("delete from t_test_parts WHERE id = ANY(:ids)")]
pub async fn delete_id_in(ids: &[i32]) {}