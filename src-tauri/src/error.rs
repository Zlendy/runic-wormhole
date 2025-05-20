use wormhole::{transfer::TransferError, WormholeError};

#[derive(Debug, thiserror::Error)]
pub enum RunicError {
    #[error(transparent)]
    WormholeError(#[from] WormholeError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    TransferError(#[from] TransferError),

    #[error("{0}")]
    StringError(String),

    #[error("Could not parse filename")]
    ParseFileNameError,
}

// we must manually implement serde::Serialize
impl serde::Serialize for RunicError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
