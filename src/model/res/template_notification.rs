use crate::model::entity::template_notification::TemplateNotification;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Clone,Deserialize)]
pub struct TemplateResponse {
    pub id: i32,
    pub code: String,
    pub title: String,
    pub content: String,
    pub template_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}


impl TemplateResponse {


    pub fn from(entity: TemplateNotification )-> Self{

        Self{
            id: entity.id,
            code: entity.code,
            title: entity.title,
            content: entity.content,
            template_type: entity.template_type,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}