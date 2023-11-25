use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SerializeError<E> {
    Unimplemented,
    DataSouceCaused(String),
    MissingGeometryField,
    NotAGeometryField(&'static str),
    InvalidGeometryContainer {
        name: &'static str,
        expected: &'static str,
    },
    PropertySinkCaused(E),
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
            InvalidGeometryContainer { name, expected } => {
                write!(f, "expected container type: {}, actual: {}", expected, name)
            }
            PropertySinkCaused(e) => e.fmt(f),
        }
    }
}
impl<E: Error> Error for SerializeError<E> {}
