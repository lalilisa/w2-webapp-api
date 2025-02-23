use crate::common::kafka::KafkaHandlerMessage;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;
use tracing::{error, info};

pub struct KafkaConsumer {
    consumer: Consumer,
}

impl KafkaConsumer {
    pub fn new(broker: &str, topic: &str, group: &str) -> Self {
        let consumer = Consumer::from_hosts(vec![broker.to_owned()])
            .with_topic(topic.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(group.to_owned())
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .expect("Consumer creation failed");

        KafkaConsumer { consumer }
    }

    pub async fn start_consuming(
        mut self,
        handler: Arc<impl KafkaHandlerMessage + Sync + Send + 'static>,
    ) {
        task::spawn(async move {
            loop {
                match self.consumer.poll() {
                    Ok(mss) => {
                        for ms in mss.iter() {
                            for m in ms.messages() {
                                let payload = String::from_utf8_lossy(m.value);

                                info!(
                                    "Received message from topic: {}, partition: {} : {:?}",
                                    ms.topic(),
                                    ms.partition(),
                                    payload
                                );
                                let handle_message =handler.handle_message(payload.to_string()).await;

                                match handle_message{
                                    Err(_err) => error!("Failed to handle message: {}", _err),
                                    Ok(_) => info!("Received message successfully"),
                                }
                                // match serde_json::from_str::<HashMap<String, Value>>(&payload) {
                                //     Ok(parsed_message) => {
                                //     }
                                //     Err(e) => {
                                //         error!("Failed to parse message: {}", e);
                                //     }
                                // }
                            }

                            self.consumer.consume_messageset(ms).unwrap();
                        }

                        self.consumer.commit_consumed().unwrap();
                    }
                    Err(e) => {
                        error!("Error polling messages: {}", e);
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    }
}
