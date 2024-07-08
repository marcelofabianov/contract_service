use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PostgresError {
    ConnectionError(String),
    // ...
}

impl fmt::Display for PostgresError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostgresError::ConnectionError(msg) => {
                write!(f, "PostgreSQL Connection Error: {}", msg)
            } // ...
        }
    }
}

impl Error for PostgresError {}
