use crate::common::kafka::{KafkaHandlerMessage, KafkaMessageData};
use crate::function;
use crate::model::req::notification::SendNotification;
use std::future::Future;
use std::pin::Pin;
use tracing::{error};

pub struct NotificationConsumerHandler;

impl NotificationConsumerHandler {
    async fn handle_action(kafka_message_data: KafkaMessageData) -> anyhow::Result<()> {
        let action = kafka_message_data.action.as_str();
        match action {
            "PUSH_NOTIFICATION" => {
                if let Some(payload) = kafka_message_data.payload {
                    let json_value = serde_json::Value::Object(payload.into_iter().collect());
                    let req: SendNotification = serde_json::from_value(json_value)?;
                    let res = function::notification::send_notification(&req)
                        .await;

                    if let Err(e) = res {
                        error!("Failed to send notification: {}", e);
                    }

                }
                else {
                    error!("PUSH_NOTIFICATION ACTION expected a payload");
                }
            }
            _ => error!("Unknown action: {}", action),
        }
        Ok(())
    }
}

impl KafkaHandlerMessage for NotificationConsumerHandler {
     fn handle_message(
        &self,
        message: String,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
       Box::pin(async move {
            let kafka_message_data = serde_json::from_str::<KafkaMessageData>(&message)?;
            Self::handle_action(kafka_message_data).await?;
            Ok(())
        })
    }
}
