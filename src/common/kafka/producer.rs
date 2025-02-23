use kafka::producer::{Producer, Record, RequiredAcks};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use once_cell::sync::OnceCell;
use tracing::{error, info};


pub struct KafkaProducer {
    producer: Producer
}

impl KafkaProducer {
    pub fn new(broker: &str) -> Self {
        let producer = Producer::from_hosts(vec![broker.to_owned()])
            .with_ack_timeout(Duration::from_secs(100))
            .with_required_acks(RequiredAcks::One)
            .create()
            .expect("Producer creation failed");

        KafkaProducer {
            producer
        }
    }

    pub fn send_message<T>(&mut self, topic: &str, message: &T)
    where
        T: Serialize,
    {
        match serde_json::to_string(&message) {
            Ok(json) => {
                self.producer
                    .send(&Record::from_value(topic, json.as_bytes()))
                    .expect("Failed to send message");

                info!("Message sent to topic {}: {}", topic, json);
            }
            Err(e) => {
                error!("Failed to serialize message: {}", e);
            }
        }
    }
}


static KAFKA_PRODUCER: OnceCell<Arc<Mutex<KafkaProducer>>> = OnceCell::new();
pub fn init_kafka_producer(broker: &str) {
    let producer = KAFKA_PRODUCER.set(Arc::new(Mutex::new(KafkaProducer::new(broker))));
    if let Err(_) = producer {
        error!("Failed to init kafka producer");
    }
}

pub fn get_kafka_producer() -> Arc<Mutex<KafkaProducer>> {
    KAFKA_PRODUCER.get().expect("Kafka producer is not initialized").clone()
}