use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

#[derive(Clone)]
pub struct Producer {
    producer: FutureProducer,
}

impl Producer {
    pub fn new(brokers: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Producer { producer }
    }

    pub async fn publisher(&self, event_json: String, topic: String) -> Result<(), String> {
        let record = FutureRecord::to(topic.as_str())
            .payload(&event_json)
            .key("default_key");

        self.producer
            .send(record, Timeout::After(Duration::from_secs(1)))
            .await
            .map(|_| ())
            .map_err(|(err, _)| format!("Kafka error: {:?}", err))
    }
}
