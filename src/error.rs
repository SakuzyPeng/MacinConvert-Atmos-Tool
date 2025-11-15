use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid channel configuration: {0}")]
    InvalidChannelConfig(String),

    #[error("Format detection failed: {0}")]
    FormatDetectionFailed(String),

    #[error("Dolby tools not found: {0}")]
    ToolsNotFound(String),

    #[error("GStreamer execution failed: {0}")]
    GStreamerFailed(String),

    #[error("Audio merge failed: {0}")]
    MergeFailed(String),

    #[error("FLAC conversion failed: {0}")]
    FlacConversionFailed(String),
}

pub type Result<T> = std::result::Result<T, DecodeError>;
