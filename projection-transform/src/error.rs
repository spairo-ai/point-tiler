use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("outside projection domain")]
    OutsideProjectionDomain,

    #[error("unsupported CRS transformation from EPSG:{from} to EPSG:{to}")]
    UnsupportedCrs { from: u16, to: u16 },

    #[error("projection error: {0}")]
    ProjectionError(String),
}
