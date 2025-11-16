use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("invalid tile coordinates: z={z}, x={x}, y={y}")]
    InvalidTileCoordinates { z: u8, x: u32, y: u32 },

    #[error("tile has no parent: z={z}")]
    TileHasNoParent { z: u8 },

    #[error("coordinate out of range: {reason}")]
    CoordinateOutOfRange { reason: String },

    #[error("failed to generate glTF: {reason}")]
    GltfGenerationFailed { reason: String },

    #[error("failed to write tile: {reason}")]
    TileWriteFailed { reason: String },

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ExportError>;
