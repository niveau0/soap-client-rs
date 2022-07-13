//! Error types for SOAP client runtime

use thiserror::Error;

/// Result type for SOAP operations
pub type SoapResult<T> = std::result::Result<T, SoapError>;

/// Errors that can occur during SOAP operations
#[derive(Error, Debug)]
pub enum SoapError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlError(String),

    /// SOAP fault received from server
    #[error("SOAP fault: {code} - {message}")]
    SoapFault {
        code: String,
        message: String,
        detail: Option<String>,
    },

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Invalid response format
    #[error("Invalid SOAP response: {0}")]
    InvalidResponse(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Other errors
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl From<quick_xml::Error> for SoapError {
    fn from(err: quick_xml::Error) -> Self {
        SoapError::XmlError(err.to_string())
    }
}
