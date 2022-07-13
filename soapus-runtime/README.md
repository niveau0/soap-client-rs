# soapus-runtime

[![Crates.io](https://img.shields.io/crates/v/soapus-runtime.svg)](https://crates.io/crates/soapus-runtime)
[![Documentation](https://docs.rs/soapus-runtime/badge.svg)](https://docs.rs/soapus-runtime)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Runtime library for SOAP clients. Handles HTTP requests, SOAP envelope building, and error handling.

**Part of the [soapus](https://github.com/niveau0/soapus) project.**

## Installation

```toml
[dependencies]
soapus-runtime = "0.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

## Quick Example

```rust
use soapus_runtime::{SoapClient, SoapResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct MyRequest {
    field: String,
}

#[derive(Deserialize)]
struct MyResponse {
    result: i32,
}

#[tokio::main]
async fn main() -> SoapResult<()> {
    let client = SoapClient::new("http://example.com/soap");
    
    let response: MyResponse = client
        .call_with_soap_action(
            "MyOperation",
            Some("http://example.com/MyOperation"),
            Some("http://tempuri.org/"),
            &MyRequest { field: "value".to_string() },
        )
        .await?;
    
    Ok(())
}
```

## Documentation

📚 **Full documentation and examples in the [main repository](https://github.com/niveau0/soapus)**:

- [Quick Start Guide](https://github.com/niveau0/soapus/blob/main/QUICKSTART.md) - Get started in 5 minutes
- [Working Examples](https://github.com/niveau0/soapus/tree/main/examples) - Real-world usage
- [Tracing Guide](https://github.com/niveau0/soapus/blob/main/docs/TRACING.md) - Observability setup
- [API Documentation](https://docs.rs/soapus-runtime) - Complete API reference

## Features

### Default Features
- `tracing` - Structured logging and distributed tracing support

### Optional Features
- `opentelemetry` - OpenTelemetry/Jaeger integration
- `metrics` - Prometheus metrics collection

Disable default features:
```toml
soapus-runtime = { version = "0.1", default-features = false }
```

### Capabilities

- ✅ SOAP 1.1 & 1.2 support
- ✅ Async/await with tokio
- ✅ Type-safe requests/responses
- ✅ Automatic envelope building
- ✅ SOAP fault detection
- ✅ Configurable timeouts
- ✅ Custom HTTP client support

## Code Generation

To generate SOAP clients from WSDL, use [`soapus-codegen`](https://crates.io/crates/soapus-codegen) in your `build.rs`.

## License

Licensed under the [MIT License](https://opensource.org/licenses/MIT).