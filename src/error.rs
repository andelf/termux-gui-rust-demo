//! Error types for Termux GUI

use std::io;
use thiserror::Error;

/// Result type alias for Termux GUI operations
pub type Result<T> = std::result::Result<T, GuiError>;

/// Errors that can occur when using Termux GUI
#[derive(Error, Debug)]
pub enum GuiError {
    /// IO error occurred
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Socket binding failed
    #[error("Failed to bind socket: {0}")]
    SocketBind(String),
    
    /// Connection to Termux GUI service failed
    #[error("Failed to connect to Termux GUI service")]
    ConnectionFailed,
    
    /// Invalid response from GUI service
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    /// View not found
    #[error("View with ID {0} not found")]
    ViewNotFound(i64),
    
    /// Invalid view operation
    #[error("Invalid view operation: {0}")]
    InvalidOperation(String),
    
    /// Event handling error
    #[error("Event handling error: {0}")]
    EventError(String),
}
