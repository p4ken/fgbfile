use std::{error::Error, fmt::Display};

use serde::ser;

#[derive(Debug)]
pub enum SerializeError {
    Unimplemented,
    DataSouceCaused(String),
}

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::DataSouceCaused(msg.to_string())
    }
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unimplemented => f.write_str("not implemented"),
            Self::DataSouceCaused(msg) => f.write_str(&msg),
        }
    }
}

impl Error for SerializeError {}
