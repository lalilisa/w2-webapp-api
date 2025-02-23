use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SendNotification {
   pub template_code : String,
   pub user_id: i32,
   pub params: HashMap<String,Value>
}