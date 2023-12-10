use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SerializeError<E> {
    DataSouceCaused(String),
    MalformedFeature,
    NoGeometryField,
    InvalidGeometryContainer {
        expected: Option<&'static str>,
        actual: &'static str,
    },
    GeometrySinkCaused(E),
    PropertySinkCaused(E),
    FeatureSinkCaused(E),
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
            DataSouceCaused(msg) => f.write_str(&msg),
            MalformedFeature => f.write_str("feature must be a struct"),
            NoGeometryField => f.write_str("feature has no geometry field"),
            InvalidGeometryContainer {
                actual,
                expected: Some(expected),
            } => {
                write!(
                    f,
                    "expected container type: {}, actual: {}",
                    expected, actual
                )
            }
            InvalidGeometryContainer {
                actual,
                expected: None,
            } => {
                write!(f, "unexpected type: {}", actual)
            }
            GeometrySinkCaused(e) => e.fmt(f),
            PropertySinkCaused(e) => e.fmt(f),
            FeatureSinkCaused(e) => e.fmt(f),
        }
    }
}
impl<E: Error> Error for SerializeError<E> {}
