//! # SOAP Client Runtime
//!
//! Runtime library for SOAP clients generated from WSDL files. Handles HTTP requests,
//! SOAP envelope building, XML serialization, and error handling.
//!
//! ## Features
//!
//! - **SOAP 1.1 & 1.2** - Full support for both SOAP versions
//! - **Async/Await** - Built on `tokio` and `reqwest` for modern async Rust
//! - **Type-Safe** - Generic over request/response types with serde
//! - **Envelope Building** - Automatic SOAP envelope construction with namespaces
//! - **Error Handling** - Comprehensive error types for all failure modes
//! - **SOAP Fault Detection** - Automatic parsing and handling of SOAP faults
//! - **Configurable** - Builder pattern for timeouts, custom HTTP clients, etc.
//! - **Observability** - Optional tracing and metrics support
//!
//! ## Basic Usage
//!
//! ```no_run
//! use soapus_runtime::{SoapClient, SoapResult};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize)]
//! struct MyRequest {
//!     field: String,
//! }
//!
//! #[derive(Deserialize)]
//! struct MyResponse {
//!     result: i32,
//! }
//!
//! #[tokio::main]
//! async fn main() -> SoapResult<()> {
//!     let client = SoapClient::new("http://example.com/soap");
//!
//!     let request = MyRequest {
//!         field: "value".to_string(),
//!     };
//!
//!     let response: MyResponse = client
//!         .call_with_soap_action(
//!             "MyOperation",
//!             Some("http://example.com/MyOperation"),
//!             Some("http://tempuri.org/"),
//!             true,
//!             &MyRequest { field: "value".to_string() },
//!         )
//!         .await?;
//!
//!     println!("Result: {}", response.result);
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ```no_run
//! use soapus_runtime::{SoapClient, SoapVersion};
//! use std::time::Duration;
//!
//! let client = SoapClient::builder("http://example.com/soap")
//!     .timeout(Duration::from_secs(30))
//!     .soap_version(SoapVersion::Soap12)
//!     .build();
//! ```
//!
//! ## Features
//!
//! - `tracing` (default) - Structured logging and distributed tracing support
//! - `opentelemetry` - OpenTelemetry/Jaeger integration for distributed tracing
//! - `metrics` - Prometheus metrics collection

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rustdoc::broken_intra_doc_links)]
// Note: missing_docs is intentionally not enabled for internal structures

pub mod client;
pub mod envelope;
pub mod error;

pub use client::SoapClient;
pub use envelope::{SoapEnvelope, SoapVersion};
pub use error::{SoapError, SoapResult};

// Re-export commonly used types
pub use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soap_client_creation() {
        let client = SoapClient::new("http://example.com/soap");
        assert_eq!(client.endpoint(), "http://example.com/soap");
    }
}
