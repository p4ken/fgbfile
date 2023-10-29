use std::{error::Error, fmt::Display};

use serde::ser;

#[derive(Debug)]
pub enum FgbFileError {
    Unimplemented,
    InputDataError(String),
}

impl ser::Error for FgbFileError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::InputDataError(msg.to_string())
    }
}

impl Display for FgbFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unimplemented => f.write_str("not implemented"),
            Self::InputDataError(msg) => f.write_str(&msg),
        }
    }
}

impl Error for FgbFileError {}
