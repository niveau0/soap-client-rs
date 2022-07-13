# SOAP Client Examples

This directory contains examples demonstrating how to use the `soapus` crate.

## 📚 Examples

### 1. Calculator Example (`calculator/`)

A simple example that demonstrates calling a real public SOAP web service.

- **Service**: http://www.dneonline.com/calculator.asmx
- **Operations**: Add, Subtract, Multiply, Divide
- **Features**: Basic SOAP calls, tracing integration

**Run it:**
```bash
cd examples/calculator
cargo run
```

### 2. Observability Example (`observability/`)

Advanced example showing full observability stack integration.

- **Features**: OpenTelemetry, Jaeger tracing, Prometheus metrics
- **Web Server**: Axum-based metrics endpoint
- **Stack**: Requires Docker for Jaeger + Prometheus

**Run it:**
```bash
cd examples/observability
docker-compose up -d  # Start Jaeger + Prometheus
cargo run
```

Then visit:
- Jaeger UI: http://localhost:16686
- Prometheus: http://localhost:9090
- Metrics endpoint: http://localhost:3000/metrics

---

## 🛠️ IDE Setup (Important!)

Both examples use **build-time code generation** from WSDL files. This means the SOAP client code is generated when you run `cargo build`, not before.

### First-Time Setup

When you first clone the repository or clean the build artifacts, your IDE (rust-analyzer) will show errors because the generated files don't exist yet.

**To fix this:**

1. **Build each example once:**
   ```bash
   cargo build -p calculator-example
   cargo build -p observability-example
   ```

2. **Restart your IDE's language server** (rust-analyzer)
   - VS Code: `Cmd/Ctrl + Shift + P` → "Rust Analyzer: Restart Server"
   - Other IDEs: Follow their restart procedure

3. **IDE errors should now be gone!**

The generated files are located at:
- `target/debug/build/calculator-example-*/out/soap_client.rs`
- `target/debug/build/observability-example-*/out/soap_client.rs`

### Why This Happens

Each example's `main.rs` contains:
```rust
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));
```

This includes code that only exists **after** the build script runs. Until then, rust-analyzer can't find the file and shows errors. This is normal and expected behavior for build-time code generation.

**Note:** You must build the examples at least once before the IDE will work correctly. This is the same behavior as other build-script-based code generation tools (e.g., prost, tonic).

---

## 📝 How the Examples Work

### Code Generation Flow

1. **build.rs** reads the WSDL file
2. **soapus-codegen** parses WSDL and generates Rust code
3. Generated code is written to `OUT_DIR`
4. **main.rs** includes the generated code directly via `include!()` macro

### Generated Code Structure

Each example generates:
- **Request/Response structs** - Type-safe Rust structs for each operation
- **Client struct** - Async methods for calling SOAP operations
- **Constants** - Namespace URIs and other WSDL metadata

Example generated code:
```rust
// Generated from WSDL
pub struct Add {
    pub int_a: i32,
    pub int_b: i32,
}

pub struct AddResponse {
    pub add_result: i32,
}

pub struct Calculator {
    client: SoapClient,
}

impl Calculator {
    pub fn new(endpoint: &str) -> Self { ... }
    pub async fn add(&self, request: Add) -> SoapResult<AddResponse> { ... }
    // ... other operations
}
```

### Using in Your Own Project

See each example's `build.rs` file for how to integrate into your own project:

```rust
// build.rs
use soap_client_codegen::SoapClientGenerator;
use std::path::PathBuf;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let wsdl_path = PathBuf::from(&manifest_dir).join("service.wsdl");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    SoapClientGenerator::builder()
        .wsdl_path(wsdl_path.to_str().unwrap())
        .out_dir(&out_dir)
        .generate()
        .expect("Failed to generate SOAP client");
}
```

---

## 🐛 Troubleshooting

### "Cannot find type `Calculator` in this scope"
### "failed to load file ... soap_client.rs"

**Solution**: Run `cargo build` first to generate the code, then restart rust-analyzer.

These errors appear because the code is generated at build time. After building once, rust-analyzer will be able to find the generated types.

### Generated code not updating after WSDL changes

**Solution**: Run `cargo clean -p <example-name>` to force regeneration:
```bash
cargo clean -p calculator-example
cargo build -p calculator-example
```

### IDE still shows errors after building

**Solution**: Restart rust-analyzer or your IDE.

---

## 📖 More Information

- **Project README**: `../../README.md`
- **Quick Start Guide**: `../../QUICKSTART.md`
- **Tracing Guide**: `../../docs/TRACING.md`
- **TODO/Roadmap**: `../../TODO.md`

---

**Happy SOAP-ing!** 🧼✨