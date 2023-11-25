use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SerializeError<E> {
    Unimplemented,
    DataSouceCaused(String),
    MissingGeometryField,
    NotAGeometryField(&'static str),
    InvalidTypeName(&'static str),
    // #[cfg(feature = "geozero")]
    GeozeroError(E),
    PropertySinkCaused(E),
}
// #[cfg(feature = "geozero")]
impl From<geozero::error::GeozeroError> for SerializeError<geozero::error::GeozeroError> {
    fn from(value: geozero::error::GeozeroError) -> Self {
        Self::GeozeroError(value)
    }
}
impl<E: Error> serde::ser::Error for SerializeError<E> {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::DataSouceCaused(msg.to_string())
    }
}
impl<E: Display> Display for SerializeError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SerializeError::*;
        match self {
            Unimplemented => f.write_str("not implemented"),
            DataSouceCaused(msg) => f.write_str(&msg),
            MissingGeometryField => f.write_str("geometry field is missing"),
            NotAGeometryField(name) => write!(f, "field {} is not a geometry", name),
            InvalidTypeName(name) => write!(f, "invalid field name {}", name),
            // #[cfg(feature = "geozero")]
            GeozeroError(e) => e.fmt(f),
            PropertySinkCaused(e) => e.fmt(f),
        }
    }
}
impl<E: Error> Error for SerializeError<E> {}
