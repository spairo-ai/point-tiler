use thiserror::Error;
use projection_transform::crs::EpsgCode;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("failed to create PROJ transformer from EPSG:{from} to EPSG:{to}: {reason}")]
    ProjTransformCreationFailed {
        from: EpsgCode,
        to: EpsgCode,
        reason: String,
    },

    #[error("PROJ transformation failed: {reason}")]
    ProjTransformFailed { reason: String },

    #[error("unsupported output CRS: EPSG:{epsg}")]
    UnsupportedOutputCrs { epsg: EpsgCode },

    #[error("coordinate transformation failed: {reason}")]
    TransformationFailed { reason: String },

    #[error("projection error: {0}")]
    ProjectionError(#[from] projection_transform::error::Error),
}

pub type Result<T> = std::result::Result<T, TransformError>;
