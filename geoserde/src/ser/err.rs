use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SerializeError<E> {
    SouceCaused(String),
    SinkCaused(E),
    NoGeometryField,
    InvalidFeatureStructure,
    InvalidGeometryStructure {
        expected: Option<&'static str>,
        actual: &'static str,
    },
    InvalidState,
}
impl<E: Error> serde::ser::Error for SerializeError<E> {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SouceCaused(msg.to_string())
    }
}
impl<E: Display> Display for SerializeError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SerializeError::*;
        match self {
            SouceCaused(msg) => f.write_str(&msg),
            SinkCaused(e) => e.fmt(f),
            NoGeometryField => f.write_str("feature has no geometry field"),
            InvalidFeatureStructure => f.write_str("feature must be a struct"),
            InvalidGeometryStructure {
                actual,
                expected: Some(expected),
            } => {
                write!(
                    f,
                    "expected container type: {}, actual: {}",
                    expected, actual
                )
            }
            InvalidGeometryStructure {
                actual,
                expected: None,
            } => {
                write!(f, "unexpected type: {}", actual)
            }
            InvalidState => f.write_str("invalid internal state"),
        }
    }
}
impl<E: Error> Error for SerializeError<E> {}
