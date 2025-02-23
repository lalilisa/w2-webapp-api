use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub mod consumer;
pub mod producer;


pub trait KafkaHandlerMessage {
     fn handle_message(&self, message: String) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KafkaMessageData {
    pub action: String,
    pub payload: Option<HashMap<String, Value>>,
}

// impl<T> KafkaMessageData<T> {
//
// }
