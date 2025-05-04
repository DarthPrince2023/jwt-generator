use std::{fmt::Display, env::VarError};
use hmac::digest::InvalidLength;
use serde_json::Error as DeserializeError;

#[derive(Debug)]
pub enum Error {
    EnvLoadError(VarError),
    InvalidDigestLength(InvalidLength),
    DeserializationError(DeserializeError)
}

impl<'b> Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EnvLoadError(error) => write!(f, "Error loading environment => {error}"),
            Self::InvalidDigestLength(error) => write!(f, "Invalid digest length => {error}"),
            Self::DeserializationError(error) => write!(f, "Could not deserialize payload => {error}")
        }
    }
}