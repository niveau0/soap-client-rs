//! Error types for SOAP client code generation

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Result type for code generation operations
pub type Result<T> = std::result::Result<T, CodegenError>;

/// Errors that can occur during SOAP client code generation
#[derive(Error, Debug)]
pub enum CodegenError {
    /// Failed to read a file
    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Failed to write a file
    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlParse(String),

    /// WSDL parsing error
    #[error("WSDL parsing error: {0}")]
    WsdlParse(String),

    /// XSD schema parsing error
    #[error("XSD schema parsing error: {0}")]
    XsdParse(String),

    /// Missing required element in WSDL
    #[error("Missing required WSDL element: {element}")]
    MissingWsdlElement { element: String },

    /// Missing required attribute in WSDL
    #[error("Missing required attribute '{attribute}' in element '{element}'")]
    MissingAttribute { element: String, attribute: String },

    /// Invalid WSDL structure
    #[error("Invalid WSDL structure: {0}")]
    InvalidWsdl(String),

    /// Invalid XSD schema
    #[error("Invalid XSD schema: {0}")]
    InvalidSchema(String),

    /// Unsupported WSDL feature
    #[error("Unsupported WSDL feature: {feature}")]
    UnsupportedFeature { feature: String },

    /// Unsupported SOAP binding style
    #[error("Unsupported SOAP binding style: {style}")]
    UnsupportedBindingStyle { style: String },

    /// Unsupported XSD type
    #[error("Unsupported XSD type: {type_name}")]
    UnsupportedType { type_name: String },

    /// Type not found in schema
    #[error("Type '{type_name}' not found in schema")]
    TypeNotFound { type_name: String },

    /// Message not found
    #[error("Message '{message_name}' not found")]
    MessageNotFound { message_name: String },

    /// Binding not found
    #[error("Binding '{binding_name}' not found")]
    BindingNotFound { binding_name: String },

    /// Port type not found
    #[error("Port type '{port_type_name}' not found")]
    PortTypeNotFound { port_type_name: String },

    /// Missing configuration field
    #[error("Missing required configuration field: {field}")]
    MissingConfiguration { field: String },

    /// Code generation error
    #[error("Code generation failed: {0}")]
    CodeGeneration(String),

    /// Invalid identifier
    #[error("Invalid Rust identifier: '{identifier}'")]
    InvalidIdentifier { identifier: String },

    /// Namespace resolution error
    #[error("Failed to resolve namespace prefix '{prefix}'")]
    NamespaceResolution { prefix: String },

    /// Import/Include error
    #[error("Failed to process import/include from '{uri}': {reason}")]
    ImportError { uri: String, reason: String },

    /// Multiple definitions with same name
    #[error("Duplicate definition: '{name}' is defined multiple times")]
    DuplicateDefinition { name: String },

    /// Generic error with context
    #[error("{context}: {source}")]
    WithContext {
        context: String,
        #[source]
        source: Box<CodegenError>,
    },

    /// Other errors
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl CodegenError {
    /// Add context to an error
    pub fn with_context(self, context: impl Into<String>) -> Self {
        CodegenError::WithContext {
            context: context.into(),
            source: Box::new(self),
        }
    }
}

impl From<quick_xml::Error> for CodegenError {
    fn from(err: quick_xml::Error) -> Self {
        CodegenError::XmlParse(err.to_string())
    }
}

impl From<std::str::Utf8Error> for CodegenError {
    fn from(err: std::str::Utf8Error) -> Self {
        CodegenError::XmlParse(format!("UTF-8 decoding error: {}", err))
    }
}

impl From<std::string::FromUtf8Error> for CodegenError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        CodegenError::XmlParse(format!("UTF-8 decoding error: {}", err))
    }
}

// Helper macro for creating errors with context
#[macro_export]
macro_rules! codegen_err {
    ($variant:ident, $($arg:tt)*) => {
        $crate::error::CodegenError::$variant(format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CodegenError::MissingWsdlElement {
            element: "types".to_string(),
        };
        assert_eq!(err.to_string(), "Missing required WSDL element: types");
    }

    #[test]
    fn test_with_context() {
        let err = CodegenError::TypeNotFound {
            type_name: "MyType".to_string(),
        };
        let err_with_ctx = err.with_context("While processing operation 'GetData'");

        assert!(err_with_ctx
            .to_string()
            .contains("While processing operation"));
        assert!(err_with_ctx.to_string().contains("MyType"));
    }
}
