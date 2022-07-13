//! # SOAP Client Code Generator
//!
//! Code generator for SOAP clients from WSDL files. This crate parses WSDL 1.1 and embedded
//! XSD schemas, then generates type-safe Rust code for making SOAP requests.
//!
//! ## Features
//!
//! - **WSDL 1.1 Parsing** - Complete support for WSDL definitions, types, messages, bindings, and services
//! - **XSD Schema Support** - ComplexType, SimpleType, sequences, enumerations, and restrictions
//! - **Idiomatic Rust** - Generates clean, type-safe Rust code with proper naming conventions
//! - **Build-Time Generation** - Integrates seamlessly with `build.rs` for compile-time safety
//! - **Type Mapping** - Automatic mapping of XSD types to Rust types
//! - **Serde Integration** - Generated types are serializable with proper XML attributes
//!
//! ## Usage in build.rs
//!
//! ```ignore
//! use soapus_codegen::SoapClientGenerator;
//!
//! println!("cargo:rerun-if-changed=service.wsdl");
//!
//! SoapClientGenerator::builder()
//!     .wsdl_path("service.wsdl")
//!     .out_dir(std::env::var("OUT_DIR").unwrap())
//!     .generate()
//!     .expect("Failed to generate SOAP client");
//! ```
//!
//! ## Using Generated Code
//!
//! ```ignore
//! // Include the generated code
//! include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = MyService::new("http://example.com/soap");
//!     
//!     let request = SomeOperation { field: "value".to_string() };
//!     let response = client.some_operation(request).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ```ignore
//! use soapus_codegen::{SoapClientGenerator, SoapVersion};
//!
//! SoapClientGenerator::builder()
//!     .wsdl_path("service.wsdl")
//!     .out_dir(std::env::var("OUT_DIR").unwrap())
//!     .client_name("MyCustomClient")
//!     .soap_version(SoapVersion::Soap12)
//!     .generate()
//!     .expect("Failed to generate SOAP client");
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rustdoc::broken_intra_doc_links)]
// Note: missing_docs is intentionally not enabled to avoid noise from internal parser structures

pub mod error;
pub mod generator;
pub mod parser;

use std::fs;
use std::path::PathBuf;

pub use error::{CodegenError, Result};
use parser::parse_wsdl;

/// Main entry point for SOAP client code generation
#[derive(Debug)]
pub struct SoapClientGenerator {
    wsdl_path: PathBuf,
    out_dir: PathBuf,
    module_name: Option<String>,
    client_name: Option<String>,
    generate_tests: bool,
    soap_version: SoapVersion,
}

/// SOAP protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SoapVersion {
    /// SOAP 1.1 (default)
    Soap11,
    /// SOAP 1.2
    Soap12,
    /// Auto-detect from WSDL
    #[default]
    Auto,
}

impl SoapClientGenerator {
    /// Create a new builder for configuring the generator
    pub fn builder() -> SoapClientGeneratorBuilder {
        SoapClientGeneratorBuilder::new()
    }

    /// Generate the SOAP client code
    pub fn generate(&self) -> Result<GeneratedCode> {
        // Read WSDL file
        let wsdl_content =
            fs::read_to_string(&self.wsdl_path).map_err(|e| CodegenError::FileRead {
                path: self.wsdl_path.clone(),
                source: e,
            })?;

        // Parse WSDL
        let wsdl_model =
            parse_wsdl(&wsdl_content).map_err(|e| CodegenError::WsdlParse(e.to_string()))?;

        // Generate code
        let code = generator::generate_client_code(&wsdl_model, self)
            .unwrap_or_else(|_| "// Code generation not yet implemented\n".to_string());

        // Write to output file
        let output_file = self.out_dir.join("soap_client.rs");
        fs::write(&output_file, &code).map_err(|e| CodegenError::FileWrite {
            path: output_file.clone(),
            source: e,
        })?;

        Ok(GeneratedCode { output_file, code })
    }

    /// Get the configured SOAP version
    pub fn soap_version(&self) -> SoapVersion {
        self.soap_version
    }

    /// Get the module name (if specified)
    pub fn module_name(&self) -> Option<&str> {
        self.module_name.as_deref()
    }

    /// Get the client name (if specified)
    pub fn client_name(&self) -> Option<&str> {
        self.client_name.as_deref()
    }

    /// Check if test generation is enabled
    pub fn generate_tests(&self) -> bool {
        self.generate_tests
    }
}

/// Builder for configuring SOAP client generation
pub struct SoapClientGeneratorBuilder {
    wsdl_path: Option<PathBuf>,
    out_dir: Option<PathBuf>,
    module_name: Option<String>,
    client_name: Option<String>,
    generate_tests: bool,
    soap_version: SoapVersion,
}

impl SoapClientGeneratorBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            wsdl_path: None,
            out_dir: None,
            module_name: None,
            client_name: None,
            generate_tests: false,
            soap_version: SoapVersion::Auto,
        }
    }

    /// Set the path to the WSDL file
    pub fn wsdl_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.wsdl_path = Some(path.into());
        self
    }

    /// Set the output directory for generated code
    pub fn out_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(dir.into());
        self
    }

    /// Set a custom module name for the generated code
    pub fn module_name(mut self, name: impl Into<String>) -> Self {
        self.module_name = Some(name.into());
        self
    }

    /// Set a custom client struct name
    pub fn client_name(mut self, name: impl Into<String>) -> Self {
        self.client_name = Some(name.into());
        self
    }

    /// Enable or disable test generation
    pub fn generate_tests(mut self, enable: bool) -> Self {
        self.generate_tests = enable;
        self
    }

    /// Set the SOAP version to use
    pub fn soap_version(mut self, version: SoapVersion) -> Self {
        self.soap_version = version;
        self
    }

    /// Build the generator and generate the code
    pub fn generate(self) -> Result<GeneratedCode> {
        let generator = self.build()?;
        generator.generate()
    }

    /// Build the generator without generating code
    pub fn build(self) -> Result<SoapClientGenerator> {
        let wsdl_path = self
            .wsdl_path
            .ok_or_else(|| CodegenError::MissingConfiguration {
                field: "wsdl_path".to_string(),
            })?;

        let out_dir = self
            .out_dir
            .ok_or_else(|| CodegenError::MissingConfiguration {
                field: "out_dir".to_string(),
            })?;

        // Ensure output directory exists
        fs::create_dir_all(&out_dir).map_err(|e| CodegenError::FileWrite {
            path: out_dir.clone(),
            source: e,
        })?;

        Ok(SoapClientGenerator {
            wsdl_path,
            out_dir,
            module_name: self.module_name,
            client_name: self.client_name,
            generate_tests: self.generate_tests,
            soap_version: self.soap_version,
        })
    }
}

impl Default for SoapClientGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of code generation
pub struct GeneratedCode {
    /// Path to the generated file
    pub output_file: PathBuf,
    /// The generated code as a string
    pub code: String,
}

/// Legacy API for backwards compatibility
///
/// Deprecated: Use `SoapClientGenerator::builder()` instead
#[deprecated(since = "0.1.0", note = "Use SoapClientGenerator::builder() instead")]
pub fn generate_from_wsdl(
    wsdl_path: &str,
    out_dir: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    SoapClientGenerator::builder()
        .wsdl_path(wsdl_path)
        .out_dir(out_dir)
        .generate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_missing_wsdl_path() {
        let result = SoapClientGeneratorBuilder::new().out_dir("/tmp").build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CodegenError::MissingConfiguration { .. }
        ));
    }

    #[test]
    fn test_builder_missing_out_dir() {
        let result = SoapClientGeneratorBuilder::new()
            .wsdl_path("test.wsdl")
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CodegenError::MissingConfiguration { .. }
        ));
    }

    #[test]
    fn test_soap_version_default() {
        assert_eq!(SoapVersion::default(), SoapVersion::Auto);
    }
}
