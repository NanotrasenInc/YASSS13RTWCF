//! Various types related to errors when opening RSIs.
use image::ImageError;
use std::io::Error as IOError;
use rustc_serialize::json::BuilderError;

/// Represents an error while parsing an RSI.
#[derive(Debug)]
pub enum RsiError {
    /// If an error occured during the IO of a file.
    IO(IOError),

    /// If an error occured during the parsing of the JSON metadata.
    Json(BuilderError),

    /// If some part of the metadata is corrupt.
    Metadata(String),

    /// If the version of the RSI can't be handled by this version of the module.
    Version,

    /// If image throws an error loading one of the PNG files.
    ImageError(ImageError)
}

impl From<IOError> for RsiError {
    fn from(err: IOError) -> RsiError {
        RsiError::IO(err)
    }
}

impl From<BuilderError> for RsiError {
    fn from(err: BuilderError) -> RsiError {
        RsiError::Json(err)
    }
}

impl From<ImageError> for RsiError {
    fn from(err: ImageError) -> RsiError {
        RsiError::ImageError(err)
    }
}
