use flatgeobuf::geozero::error::GeozeroError;
use geoserde::ser::SerializeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FgbFileError {
    #[error("serialize failed")]
    Serialize(#[from] SerializeError<GeozeroError>),

    #[error("flatgeobuf failed")]
    FlatGeobuf(#[from] flatgeobuf::Error),
}
