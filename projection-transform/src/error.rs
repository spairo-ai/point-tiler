use thiserror::Error;
use crate::crs::EpsgCode;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("outside projection domain")]
    OutsideProjectionDomain,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("transformation from EPSG:{from} to EPSG:{to} is not supported: {reason}")]
    UnsupportedTransformation {
        from: EpsgCode,
        to: EpsgCode,
        reason: String,
    },

    #[error("transformation failed: {reason}")]
    TransformationFailed { reason: String },

    #[error("invalid EPSG code: {epsg}")]
    InvalidEpsgCode { epsg: EpsgCode },

    #[error("coordinate out of valid range: {reason}")]
    CoordinateOutOfRange { reason: String },

    #[error(transparent)]
    LegacyTransformError(#[from] TransformError),
}
