use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct KafkaError(pub(crate) String);

impl fmt::Display for KafkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kafka error: {}", self.0)
    }
}

impl Error for KafkaError {}
