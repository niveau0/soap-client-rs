# Tracing & Observability

This document explains how to use tracing and observability features in `soapus`.

---

## 📊 Overview

The SOAP client includes built-in support for structured logging and distributed tracing using the [`tracing`](https://docs.rs/tracing) crate. This enables:

- **Structured Logging**: Context-rich log messages with spans and events
- **Performance Monitoring**: Track request duration and identify bottlenecks
- **Debugging**: Understand what's happening during SOAP calls
- **Future Integration**: Ready for OpenTelemetry/Jaeger (Phase 2)

---

## 🎯 Quick Start

### Enable Tracing (Default)

Tracing is enabled by default. Just initialize a subscriber in your application:

```rust
use tracing_subscriber;

fn main() {
    // Initialize with environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Your code here...
}
```

### Run with Different Log Levels

```bash
# Info level (default)
cargo run

# Debug level (more details)
RUST_LOG=debug cargo run

# Trace level (very verbose)
RUST_LOG=trace cargo run

# Filter by module
RUST_LOG=soap_client_runtime=debug cargo run

# Multiple filters
RUST_LOG=soap_client_runtime=debug,soap_client_codegen=info cargo run
```

---

## 📝 What Gets Logged?

### Build-Time (Code Generation)

The code generator logs parsing and generation progress:

```
INFO  Starting WSDL parsing xml_size=12345
DEBUG Parsing definitions element
DEBUG Parsing types element
DEBUG Generating complex types complex_type_count=5
INFO  Code generation completed successfully code_size=8912
```

**Log Levels:**
- `INFO` - Major milestones (parsing started, completed)
- `DEBUG` - Detailed progress (element-by-element parsing)
- `ERROR` - Parse/generation failures
- `WARN` - Recoverable issues (currently unused in codegen)

### Runtime (SOAP Requests)

The runtime client logs HTTP requests and SOAP operations:

```
INFO  soap_operation{operation="Add" service="Calculator"}: Initiating SOAP call
DEBUG Building SOAP envelope operation=Add soap_action=Some("http://tempuri.org/Add")
DEBUG SOAP envelope built envelope_size=342
INFO  Sending HTTP POST request endpoint=http://example.com/service.asmx
DEBUG Received HTTP response status=200 OK
DEBUG Received response body response_size=512
DEBUG Parsing SOAP response
INFO  SOAP call completed successfully operation=Add
```

**Log Levels:**
- `INFO` - Request lifecycle (started, completed)
- `DEBUG` - Detailed steps (envelope building, parsing)
- `WARN` - External failures (endpoint unreachable, SOAP faults, timeouts)
- `ERROR` - Internal failures (serialization errors, invalid responses)

### Tracing Spans

Operations are instrumented with hierarchical spans:

```
soap_operation (operation="Add", service="Calculator")
  └─ call_with_soap_action (endpoint="...", soap_version=Soap11)
      ├─ Building envelope
      ├─ HTTP POST request
      └─ Parsing response
```

This enables:
- **Duration tracking** for each operation
- **Context propagation** for distributed tracing
- **Filtering** by operation, service, or endpoint

---

## 🔧 Generated Code

The code generator automatically adds tracing spans to all generated methods:

```rust
// Generated code includes tracing spans
pub async fn add(&self, request: Add) -> SoapResult<AddResponse> {
    #[cfg(feature = "tracing")]
    let _span = tracing::info_span!(
        "soap_operation",
        operation = "Add",
        service = "Calculator"
    ).entered();

    self.client.call_with_soap_action("Add", Some("..."), Some(TARGET_NAMESPACE), &request).await
}
```

This means **zero-configuration tracing** for all SOAP operations!

---

## 🎨 Example: Calculator with Tracing

See `examples/calculator/src/main.rs`:

```rust
#[tokio::main]
async fn main() -> SoapResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let calculator = Calculator::new("http://www.dneonline.com/calculator.asmx");
    
    // This will log automatically!
    let result = calculator.add(Add { int_a: 5, int_b: 3 }).await?;
    
    Ok(())
}
```

Run it:

```bash
cd examples/calculator
RUST_LOG=info cargo run
```

Output:

```
🧮 Calculator SOAP Client Example
📡 Connecting to: http://www.dneonline.com/calculator.asmx
➕ Testing Add operation: 5 + 3
2025-12-17T09:42:34.974Z  INFO soap_operation{operation="Add" service="Calculator"}:call_with_soap_action{...}: soap_client_runtime::client: Sending HTTP POST request endpoint=http://www.dneonline.com/calculator.asmx
   Result: 8
✅ All operations completed successfully!
```

---

## ⚙️ Configuration

### Disable Tracing (Opt-Out)

If you don't want tracing overhead, disable the feature:

```toml
[dependencies]
soapus-runtime = { version = "0.1", default-features = false, features = ["soap11", "soap12"] }
```

This removes all tracing code at compile time (zero overhead).

### Custom Subscriber

You can configure the subscriber with different formats:

```rust
// JSON format (for log aggregation)
tracing_subscriber::fmt()
    .json()
    .init();

// Compact format
tracing_subscriber::fmt()
    .compact()
    .init();

// With timestamps
tracing_subscriber::fmt()
    .with_target(false)
    .with_thread_ids(true)
    .with_file(true)
    .with_line_number(true)
    .init();
```

### Filter Examples

```bash
# Only warnings and errors
RUST_LOG=warn cargo run

# Debug for SOAP client, info for everything else
RUST_LOG=info,soap_client_runtime=debug cargo run

# Trace only HTTP requests
RUST_LOG=soap_client_runtime::client=trace cargo run

# Silence tracing completely
RUST_LOG=off cargo run
```

---

## 🚀 Phase 2: Advanced Observability (Planned)

Future versions will include:

### OpenTelemetry/Jaeger Integration

```toml
[dependencies]
soapus-runtime = { version = "0.2", features = ["tracing", "opentelemetry"] }
```

```rust
// Export traces to Jaeger
let tracer = opentelemetry_jaeger::new_pipeline()
    .with_service_name("my-soap-client")
    .install_simple()?;

let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
tracing_subscriber::registry()
    .with(telemetry)
    .init();
```

### Metrics Export (Prometheus)

```toml
[dependencies]
soapus-runtime = { version = "0.2", features = ["tracing", "metrics"] }
```

Expose metrics:

```rust
// Metrics available:
// - soap_requests_total{operation, service, status}
// - soap_request_duration_seconds{operation, service}
// - soap_errors_total{operation, error_type}
// - soap_response_size_bytes{operation}
```

### Observability Example

A complete example with Jaeger + Prometheus will be available in `examples/observability/`:

```bash
cd examples/observability
docker-compose up -d  # Start Jaeger + Prometheus
cargo run             # Run SOAP client with full observability
```

Then visit:
- Jaeger UI: http://localhost:16686
- Prometheus: http://localhost:9090
- Metrics endpoint: http://localhost:8080/metrics

---

## 📊 Log Level Guidelines

### When to Use Each Level

- **`ERROR`**: Internal bugs or critical failures
  - WSDL parsing failed (malformed XML)
  - Code generation failed (unsupported type)
  - Serialization failed (should never happen)

- **`WARN`**: External issues or expected failures
  - SOAP endpoint unreachable
  - HTTP request timeout
  - SOAP Fault received
  - Connection refused

- **`INFO`**: Important milestones
  - WSDL parsing started/completed
  - Code generation started/completed
  - SOAP request sent/completed
  - Service name, operation name

- **`DEBUG`**: Developer details
  - Parsing each WSDL element
  - Generating each struct/method
  - Building SOAP envelope
  - HTTP response status
  - Response body size

- **`TRACE`**: Very verbose details
  - XML element attributes
  - Serialized request body
  - Response parsing details
  - (Currently not used, reserved for future)

---

## 🛠️ Debugging Tips

### 1. See Full Request/Response Flow

```bash
RUST_LOG=soap_client_runtime=debug cargo run
```

This shows:
- Envelope building
- HTTP request sent
- Response received
- Response parsing

### 2. Find Slow Operations

Look for the duration in span output (requires a compatible subscriber):

```
soap_operation{operation="SlowOp"} completed in 2.3s
```

### 3. Trace Specific Service

```bash
RUST_LOG=soap_operation{service="MyService"}=trace cargo run
```

### 4. Debug Generated Code

The generated code is in `target/debug/build/<package>/out/soap_client.rs`.

Check that tracing spans are present:

```bash
grep "tracing::info_span" target/debug/build/*/out/soap_client.rs
```

---

## 🔍 Troubleshooting

### "No logs appear"

Make sure you initialized the subscriber:

```rust
tracing_subscriber::fmt().init();
```

### "Too verbose"

Lower the log level:

```bash
RUST_LOG=info cargo run  # Instead of debug/trace
```

### "Feature not enabled"

Ensure tracing feature is enabled (it's default):

```toml
[dependencies]
soapus-runtime = { version = "0.1", features = ["tracing"] }
```

### "Logs in production"

Use structured JSON output for log aggregation:

```rust
tracing_subscriber::fmt()
    .json()
    .with_target(true)
    .init();
```

Or disable entirely for maximum performance:

```toml
soapus-runtime = { version = "0.1", default-features = false, features = ["soap11", "soap12"] }
```

---

## 📚 Further Reading

- [Tracing Documentation](https://docs.rs/tracing)
- [Tracing Subscriber Guide](https://docs.rs/tracing-subscriber)
- [OpenTelemetry](https://opentelemetry.io/)
- [Jaeger Tracing](https://www.jaegertracing.io/)
- [Prometheus](https://prometheus.io/)

---

**Note**: This is Phase 1 implementation (basic tracing). Phase 2 (OpenTelemetry/Metrics) is planned for future releases. See `TODO.md` for roadmap.