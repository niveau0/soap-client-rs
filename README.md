# soapus

> **⚠️ Development Notice**: This SOAP client was **almost entirely AI-generated** ("vibe coded") and has undergone **manual review and testing**. While all tests pass and the calculator example works with real SOAP services, please conduct thorough testing before using in production environments.

A modern, type-safe SOAP client library for Rust that generates code from WSDL files at build time.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/niveau0/soapus/actions/workflows/ci.yml/badge.svg)](https://github.com/niveau0/soapus/actions/workflows/ci.yml)

## 🚀 Features

### Core Functionality
- **Type-Safe Code Generation**: Generate Rust structs and client code from WSDL files
- **Build-Time Generation**: Integrate seamlessly with `build.rs` for compile-time safety
- **SOAP 1.1 & 1.2 Support**: Full support for both SOAP versions
- **Async/Await**: Built on `tokio` and `reqwest` for modern async Rust
- **Serde Integration**: All generated types are serializable with `serde`
- **Comprehensive Error Handling**: Detailed error types for XML parsing, HTTP, and SOAP faults
- **Namespace Support**: Automatic handling of XML namespaces and SOAPAction headers
- **Zero Runtime Dependencies on WSDL**: Generated code is self-contained

### Observability & Monitoring
- **Distributed Tracing**: OpenTelemetry/Jaeger integration for request tracing
- **Metrics Export**: Prometheus metrics for request duration, errors, and response sizes
- **Structured Logging**: JSON and human-readable logging with `tracing` crate
- **Zero Overhead**: All observability features are optional and compile-time removable

## 📦 Project Structure

This project is organized as a Cargo workspace with multiple crates:

- **`soapus-codegen`**: Core library for parsing WSDL/XSD and generating Rust code
- **`soapus-runtime`**: Runtime library for executing SOAP requests
- **`soapus-cli`**: Command-line tool for parsing WSDL files and generating code
- **`examples/calculator`**: Working example using a public SOAP service
- **`examples/observability`**: Full observability stack with Jaeger, Prometheus, and Grafana

## 🔧 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
soapus-runtime = { git = "https://github.com/niveau0/soapus" }
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }

[build-dependencies]
soapus-codegen = { git = "https://github.com/niveau0/soapus" }
```

## 📖 Quick Start

### 1. Create a `build.rs` file

```rust
use soapus_codegen::SoapClientGenerator;

fn main() {
    println!("cargo:rerun-if-changed=service.wsdl");

    SoapClientGenerator::builder()
        .wsdl_path("service.wsdl")
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .generate()
        .expect("Failed to generate SOAP client");
}
```

### 2. Add your WSDL file

Place your WSDL file as `service.wsdl` in your project root.

### 3. Use the generated client

```rust
// Include the generated code
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with service endpoint
    let client = Calculator::new("http://www.dneonline.com/calculator.asmx");

    // Call SOAP operations
    let request = Add { int_a: 5, int_b: 3 };
    let response = client.add(request).await?;

    println!("Result: {}", response.add_result);

    Ok(())
}
```

## 🎯 Complete Working Example

See the [`examples/calculator`](examples/calculator) directory for a complete, working example that demonstrates:

- Build-time code generation from WSDL
- Making real SOAP calls to a public service
- All four basic calculator operations (Add, Subtract, Multiply, Divide)
- Proper error handling
- Async/await usage

To run the example:

```bash
cd examples/calculator
cargo run
```

Expected output:
```
🧮 Calculator SOAP Client Example

📡 Connecting to: http://www.dneonline.com/calculator.asmx

➕ Testing Add operation: 5 + 3
   Result: 8

➖ Testing Subtract operation: 10 - 4
   Result: 6

✖️  Testing Multiply operation: 7 * 6
   Result: 42

➗ Testing Divide operation: 20 / 4
   Result: 5

✅ All operations completed successfully!
🎉 SOAP client is working perfectly!
```

### Observability Example

Full observability stack with distributed tracing and metrics:

```bash
cd examples/observability

# Start Jaeger + Prometheus + Grafana
docker-compose up -d

# Run the service
cargo run
```

This example includes:
- **Distributed Tracing**: OpenTelemetry/Jaeger integration
- **Metrics**: Prometheus metrics export (`/metrics` endpoint)
- **Web Service**: Calculator API at http://localhost:3000
- **Dashboards**: Grafana pre-configured with datasources

Visit:
- Service: http://localhost:3000
- Jaeger UI: http://localhost:16686
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3001 (admin/admin)

Generate some traffic:
```bash
curl "http://localhost:3000/calculate?a=10&b=5&op=add"
```

See [examples/observability/README.md](examples/observability/README.md) for full details.

## 🏗️ How It Works

```
┌─────────────────────────────────────────┐
│         Your Rust Application           │
│  ┌─────────────────────────────────┐   │
│  │    Generated Client Code        │   │
│  │  (from build.rs + WSDL)         │   │
│  └──────────────┬──────────────────┘   │
│                 │                       │
│                 ▼                       │
│  ┌─────────────────────────────────┐   │
│  │   soapus-runtime                │   │
│  │  - SoapClient                   │   │
│  │  - Envelope Builder             │   │
│  │  - HTTP Transport               │   │
│  └──────────────┬──────────────────┘   │
└─────────────────┼─────────────────────┘
                  │
                  ▼
         ┌────────────────┐
         │  SOAP Service  │
         │   (HTTP/HTTPS) │
         └────────────────┘
```

### Build Time

1. **WSDL Parsing**: Your WSDL file is parsed to extract types, operations, and bindings
2. **XSD Processing**: XML Schema types are converted to Rust structs
3. **Code Generation**: Type-safe client code is generated with async methods
4. **Compilation**: Generated code is compiled with your project

### Runtime

1. **Request Building**: Your request struct is serialized to XML
2. **SOAP Envelope**: XML is wrapped in a SOAP envelope with proper namespaces
3. **HTTP Transport**: Request is sent via HTTPS with appropriate headers
4. **Response Parsing**: SOAP response is parsed and deserialized to Rust types

## 📝 Generated Code Example

Given a WSDL operation like:

```xml
<operation name="Add">
    <input message="tns:AddSoapIn"/>
    <output message="tns:AddSoapOut"/>
</operation>
```

The generator creates:

```rust
/// Request type for Add operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Add {
    #[serde(rename = "intA")]
    pub int_a: i32,
    #[serde(rename = "intB")]
    pub int_b: i32,
}

/// Response type for Add operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddResponse {
    #[serde(rename = "AddResult")]
    pub add_result: i32,
}

/// SOAP client
impl Calculator {
    /// Calls the Add operation
    pub async fn add(&self, request: Add) -> SoapResult<AddResponse> {
        self.client
            .call_with_soap_action(
                "Add",
                Some("http://tempuri.org/Add"),
                Some("http://tempuri.org/"),
                &request,
            )
            .await
    }
}
```

## 🛠️ Advanced Configuration

### Custom Client Name

```rust
SoapClientGenerator::builder()
    .wsdl_path("service.wsdl")
    .out_dir(std::env::var("OUT_DIR").unwrap())
    .client_name("MyCustomClient")
    .generate()?;
```

### SOAP Version Selection

```rust
use soapus_codegen::SoapVersion;

SoapClientGenerator::builder()
    .wsdl_path("service.wsdl")
    .out_dir(std::env::var("OUT_DIR").unwrap())
    .soap_version(SoapVersion::Soap12)
    .generate()?;
```

## ✅ Current Status

**Working Features:**
- ✅ WSDL 1.1 parsing (definitions, types, messages, portTypes, bindings, services)
- ✅ XSD schema parsing (complexType, simpleType, elements, sequences)
- ✅ Complete type mapping (all XSD built-in types → Rust types)
- ✅ Rust code generation (structs, enums, client)
- ✅ SOAP 1.1 and 1.2 envelope building
- ✅ XML namespace handling
- ✅ SOAPAction header support
- ✅ Async HTTP client (reqwest-based)
- ✅ SOAP fault detection and parsing
- ✅ Comprehensive error handling
- ✅ Working examples with real SOAP services

**Tested With:**
- Calculator Service (http://www.dneonline.com/calculator.asmx)
- All operations successfully tested

## 🧪 Testing

Run all tests:

```bash
cargo test --all
```

Run with logging:

```bash
RUST_LOG=debug cargo test --all
```

Current test coverage:
- Unit tests in codegen and runtime
- Integration tests: Full WSDL parsing and code generation
- End-to-end: Calculator example

## 🐛 Troubleshooting

### Build Error: "WSDL file not found"
Ensure your WSDL file path in `build.rs` is correct relative to your project root.

### Runtime Error: Connection refused
Check that the SOAP service endpoint URL is correct and accessible.

### Generated Code Compilation Errors
Inspect the generated code:
```bash
cat target/debug/build/*/out/soap_client.rs
```

## 📚 Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- [TODO](TODO.md) - Current tasks and priorities
- [soapus-cli](soapus-cli/README.md) - CLI tool for testing WSDL parsing and code generation

## 🗺️ Project Status

### ✅ Completed
- **Phase 1**: Core Functionality (WSDL/XSD parsing, code generation, SOAP runtime)
- **Phase 2**: Observability (Distributed tracing, Prometheus metrics, JSON logging)

### 🚧 Next Steps
See [TODO.md](TODO.md) for detailed task list including:
- Production readiness (WS-Security, MTOM/XOP, enhanced error handling)
- Advanced features (WS-Addressing, custom headers, CLI tool)
- Testing with more real-world WSDL files

## 🤝 Contributing

Contributions are welcome! Areas where help is needed:

- Testing with more real-world WSDL files
- WS-Security implementation
- Documentation improvements
- Performance optimization
- Additional examples

## 📄 License

Licensed under the MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).

### Dependencies

All dependencies are licensed under MIT or dual-licensed MIT/Apache-2.0, ensuring full compatibility with the MIT license.

## 🤖 AI Assistant Configuration

This project includes configuration files for AI assistants in the `.github/` directory. See `.github/copilot-instructions.md` for coding conventions and `.github/copilot-chat-context.md` for project context.

## 🙏 Acknowledgments

- Inspired by .NET's `wsdl.exe` and Java's `wsimport`
- Built on excellent Rust crates: `quick-xml`, `serde`, `tokio`, `reqwest`
- Thanks to http://www.dneonline.com for providing free public SOAP services for testing
