use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateNotificationRequest {
    pub code: String,
    pub title: String,
    pub content: String,
    pub template_type: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTemplateRequest {
    pub code: String,
    pub title: String,
    pub content: String,
    pub template_type: Option<String>,
    pub is_active: bool,
}
