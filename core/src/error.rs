use crate::models::EntryId;
use std::net::SocketAddr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Entry not found: {0}")]
    NotFound(EntryId),

    #[error("Port {0} already in use")]
    PortInUse(u16),

    #[error("Entry is already {status} (id: {id})")]
    InvalidState { id: EntryId, status: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Log rotation error: {0}")]
    LogRotation(String),

    #[error("Failed to bind address {addr}: {source}")]
    BindError { addr: SocketAddr, source: std::io::Error },

    #[error("Config directory not found")]
    ConfigDirNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;
