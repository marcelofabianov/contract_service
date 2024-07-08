use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EnvError(pub(crate) String);

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Env error: {}", self.0)
    }
}

impl Error for EnvError {}
