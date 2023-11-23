use std::{error::Error, fmt::Display};

use serde::ser;

#[derive(Debug)]
pub enum SerializeError {
    Unimplemented,
    DataSouceCaused(String),
    MissingGeometryField,
    NotAGeometryField(&'static str),
    // #[cfg(feature = "geozero")]
    GeozeroError(geozero::error::GeozeroError),
}
// #[cfg(feature = "geozero")]
impl From<geozero::error::GeozeroError> for SerializeError {
    fn from(value: geozero::error::GeozeroError) -> Self {
        Self::GeozeroError(value)
    }
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
        use SerializeError::*;
        match self {
            Unimplemented => f.write_str("not implemented"),
            DataSouceCaused(msg) => f.write_str(&msg),
            MissingGeometryField => f.write_str("geometry field is missing"),
            NotAGeometryField(name) => write!(f, "field {} is not a geometry", name),
            // #[cfg(feature = "geozero")]
            GeozeroError(e) => e.fmt(f),
        }
    }
}
impl Error for SerializeError {}
