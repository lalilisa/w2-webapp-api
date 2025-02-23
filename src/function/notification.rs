use crate::common::error::AppError;
use crate::common::response::res::MessageResponse;
use crate::model::entity::notification::Notification;
use crate::model::entity::template_notification::TemplateNotification;
use crate::model::req::notification::SendNotification;
use crate::repo;
use chrono::Utc;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub async fn send_notification(req: &SendNotification) -> Result<MessageResponse, AppError> {
    let template =
        repo::redis::get_hash_object::<TemplateNotification>("TEMPLATES", &req.template_code)
            .await?;

    let notification = Notification {
        id: 0,
        user_id: req.user_id,
        title: fill_template(&template.title, &req.params),
        content: fill_template(&template.content, &req.params),
        status: "SENT".to_string(),
        sent_at: Some(Utc::now()),
        is_read: false,
        created_at: Some(Utc::now()),
        updated_at: None,
    };

    repo::notification::create_notification(notification).await?;

    Ok(MessageResponse::new("Sent notification successfully"))
}

fn fill_template(content: &str, params: &HashMap<String, Value>) -> String {
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();

    let result = re.replace_all(content, |caps: &regex::Captures| {
        let key = &caps[1];
        match params.get(key) {
            Some(value) => match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "".to_string(),
                _ => "".to_string(),
            },
            None => "".to_string(),
        }
    });

    result.to_string()
}
