use crate::errors::EnvError;
use crate::errors::KafkaError;
use crate::errors::PostgresError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    EnvError(EnvError),
    PostgresError(PostgresError),
    KafkaError(KafkaError),
    Other(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::EnvError(err) => write!(f, "EnvError: {}", err),
            ServiceError::PostgresError(err) => write!(f, "PostgresError: {}", err),
            ServiceError::KafkaError(err) => write!(f, "KafkaError: {}", err),
            ServiceError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl StdError for ServiceError {}

impl From<EnvError> for ServiceError {
    fn from(err: EnvError) -> Self {
        ServiceError::EnvError(err)
    }
}

impl From<PostgresError> for ServiceError {
    fn from(err: PostgresError) -> Self {
        ServiceError::PostgresError(err)
    }
}

impl From<KafkaError> for ServiceError {
    fn from(err: KafkaError) -> Self {
        ServiceError::KafkaError(err)
    }
}

impl From<Box<dyn std::error::Error>> for ServiceError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ServiceError::Other(format!("Boxed error: {}", err))
    }
}
