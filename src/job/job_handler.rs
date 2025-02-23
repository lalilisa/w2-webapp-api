use std::collections::HashMap;
use std::env;
use chrono::Utc;
use log::info;
use serde_json::Value;
use crate::common::kafka::KafkaMessageData;
use crate::common::kafka::producer::KafkaProducer;
use crate::common::utils::struct_to_hashmap;
use crate::model::req::notification::SendNotification;
use crate::repo;

pub async fn remind_learning() {
    info!("Cron job executed at {:?}", Utc::now());
    let users = repo::user::find_all().await.unwrap();
    let kafka_urls = env::var("KAFKA_URLS").expect("KAFKA_URL NOT FOUND");

    let mut producer =KafkaProducer::new(&kafka_urls);

    for user in users  {
        let mut params = HashMap::new();
        params.insert("username".to_string(), Value::from(user.username.to_string()));

        let payload = SendNotification {
            template_code: "REMIND_LEARNING".to_string(),
            user_id: user.id,
            params,
        };

        let kafka_message_data = KafkaMessageData {
            action: "PUSH_NOTIFICATION".to_string(),
            payload: Some(struct_to_hashmap(&payload)),
        };
        producer.send_message("notification",&kafka_message_data);

    }
}