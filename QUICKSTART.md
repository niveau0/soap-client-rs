# Quick Start Guide - soapus

Get started with `soapus` in 5 minutes!

## 🚀 What You'll Build

A fully functional SOAP client that can call real web services, automatically generated from a WSDL file at build time.

## 📋 Prerequisites

- Rust 1.70 or later
- Basic familiarity with async Rust (tokio)

## 🎯 Quick Example

Let's create a simple calculator client that calls a real SOAP service!

### Step 1: Create Your Project

```bash
cargo new my-soap-client
cd my-soap-client
```

### Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "my-soap-client"
version = "0.1.0"
edition = "2021"

[dependencies]
soapus-runtime = { git = "https://github.com/niveau0/soapus" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }

[build-dependencies]
soapus-codegen = { git = "https://github.com/niveau0/soapus" }
```

Or if you have the project locally:

```toml
[dependencies]
soapus-runtime = { path = "../soapus/soapus-runtime" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1", features = ["derive"] }

[build-dependencies]
soapus-codegen = { path = "../soapus/soapus-codegen" }
```

### Step 3: Create `build.rs`

Create a `build.rs` file in your project root:

```rust
use soapus_codegen::SoapClientGenerator;

fn main() {
    println!("cargo:rerun-if-changed=service.wsdl");
    
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    
    SoapClientGenerator::builder()
        .wsdl_path("service.wsdl")
        .out_dir(&out_dir)
        .generate()
        .expect("Failed to generate SOAP client from WSDL");
}
```

### Step 4: Download a WSDL File

Download the calculator service WSDL:

```bash
curl -o service.wsdl http://www.dneonline.com/calculator.asmx?wsdl
```

Or create `service.wsdl` manually with this content (first few lines):
```xml
<?xml version="1.0" encoding="utf-8"?>
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" ...>
    <!-- The full WSDL from http://www.dneonline.com/calculator.asmx?wsdl -->
</wsdl:definitions>
```

### Step 5: Write Your Code

Edit `src/main.rs`:

```rust
// Include the generated SOAP client code
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));

#[tokio::main]
async fn main() -> SoapResult<()> {
    println!("🧮 Calculator SOAP Client Example\n");

    // Create the client
    let calculator = Calculator::new("http://www.dneonline.com/calculator.asmx");

    // Call the Add operation
    let request = Add { int_a: 5, int_b: 3 };
    let response = calculator.add(request).await?;
    
    println!("5 + 3 = {}", response.add_result);

    Ok(())
}
```

### Step 6: Build & Run

```bash
cargo build
cargo run
```

**Expected output:**
```
🧮 Calculator SOAP Client Example

5 + 3 = 8
```

🎉 **Congratulations!** You just made your first SOAP call with a type-safe Rust client!

---

## 🔍 What Just Happened?

### Build Time
1. **`build.rs` ran** and parsed your WSDL file
2. **Code was generated** in `$OUT_DIR/soap_client.rs`
3. **Types were created** for all SOAP operations
4. **Client struct was generated** with async methods

### Runtime
1. **Request was serialized** to XML
2. **SOAP envelope was built** with proper namespaces
3. **HTTP POST was sent** to the service endpoint
4. **Response was parsed** and deserialized to Rust types

---

## 📖 Understanding Generated Code

When you build your project, the generator creates:

### 1. Request/Response Types

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Add {
    #[serde(rename = "intA")]
    pub int_a: i32,
    #[serde(rename = "intB")]
    pub int_b: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddResponse {
    #[serde(rename = "AddResult")]
    pub add_result: i32,
}
```

### 2. SOAP Client

```rust
#[derive(Debug, Clone)]
pub struct Calculator {
    client: SoapClient,
}

impl Calculator {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            client: SoapClient::new(endpoint),
        }
    }
    
    pub async fn add(&self, request: Add) -> SoapResult<AddResponse> {
        self.client.call_with_soap_action(
            "Add",
            Some("http://tempuri.org/Add"),
            Some("http://tempuri.org/"),
            &request,
        ).await
    }
    
    // ... other operations (subtract, multiply, divide)
}
```

### 3. All SOAP Operations

Every operation in the WSDL becomes an async method:

```rust
calculator.add(Add { int_a: 5, int_b: 3 }).await?;
calculator.subtract(Subtract { int_a: 10, int_b: 4 }).await?;
calculator.multiply(Multiply { int_a: 7, int_b: 6 }).await?;
calculator.divide(Divide { int_a: 20, int_b: 4 }).await?;
```

---

## 🎨 Customization

### Custom Client Name

```rust
// In build.rs
SoapClientGenerator::builder()
    .wsdl_path("service.wsdl")
    .out_dir(std::env::var("OUT_DIR").unwrap())
    .client_name("MyCalculator")  // Custom name
    .generate()?;
```

### SOAP Version

```rust
use soapus_codegen::SoapVersion;

SoapClientGenerator::builder()
    .wsdl_path("service.wsdl")
    .out_dir(std::env::var("OUT_DIR").unwrap())
    .soap_version(SoapVersion::Soap12)  // Use SOAP 1.2
    .generate()?;
```

### Inspect Generated Code

```bash
# Find and view the generated code
find target/debug/build -name "soap_client.rs" -exec cat {} \;

# Or on Windows
dir target\debug\build\*\out\soap_client.rs /s
```

---

## 🚀 Next Steps

### Try More Operations

```rust
#[tokio::main]
async fn main() -> SoapResult<()> {
    let calc = Calculator::new("http://www.dneonline.com/calculator.asmx");

    // Addition
    let sum = calc.add(Add { int_a: 10, int_b: 5 }).await?;
    println!("10 + 5 = {}", sum.add_result);

    // Subtraction
    let diff = calc.subtract(Subtract { int_a: 10, int_b: 5 }).await?;
    println!("10 - 5 = {}", diff.subtract_result);

    // Multiplication
    let product = calc.multiply(Multiply { int_a: 10, int_b: 5 }).await?;
    println!("10 * 5 = {}", product.multiply_result);

    // Division
    let quotient = calc.divide(Divide { int_a: 10, int_b: 5 }).await?;
    println!("10 / 5 = {}", quotient.divide_result);

    Ok(())
}
```

### Error Handling

```rust
match calculator.add(request).await {
    Ok(response) => {
        println!("Result: {}", response.add_result);
    }
    Err(e) => {
        eprintln!("Error calling SOAP service: {}", e);
    }
}
```

### Use Your Own WSDL

Replace `service.wsdl` with any WSDL file:

```bash
# Download any public WSDL
curl -o service.wsdl https://your-service.com/service?wsdl

# Or copy a local file
cp /path/to/your.wsdl service.wsdl

# Rebuild
cargo build
```

The generator will automatically create types and methods for all operations!

---

## 🔧 Working Example

See the complete working example in [`examples/calculator`](examples/calculator):

```bash
cd examples/calculator
cargo run
```

This demonstrates:
- ✅ Build-time code generation
- ✅ All four calculator operations
- ✅ Proper error handling
- ✅ Real SOAP calls to a public service

---

## 🐛 Troubleshooting

### "WSDL file not found" Error

```rust
// In build.rs, use absolute path if needed
let wsdl_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("service.wsdl");

SoapClientGenerator::builder()
    .wsdl_path(wsdl_path.to_str().unwrap())
    .out_dir(std::env::var("OUT_DIR").unwrap())
    .generate()?;
```

### "Generated code has errors"

```bash
# Check the generated code
cat target/debug/build/*/out/soap_client.rs

# Enable verbose build output
cargo build -vv
```

### Runtime Connection Errors

```rust
// Check the endpoint URL
let calculator = Calculator::new("http://www.dneonline.com/calculator.asmx");

// Add timeout configuration (if needed in future)
// let calculator = Calculator::new(endpoint).with_timeout(Duration::from_secs(30));
```

---

## 📚 Learn More

- **[README.md](README.md)** - Full project overview
- **[TODO.md](TODO.md)** - Current tasks and future features
- **[examples/calculator](examples/calculator)** - Complete working example

---

## ✨ Features You Get

- ✅ **Type Safety**: Compile-time checks for all SOAP calls
- ✅ **Async/Await**: Modern async Rust with tokio
- ✅ **Automatic Serialization**: No manual XML writing
- ✅ **Error Handling**: Comprehensive error types
- ✅ **Namespace Support**: Automatic XML namespace handling
- ✅ **SOAPAction Headers**: Automatically set from WSDL
- ✅ **Zero Runtime WSDL**: Generated code is self-contained

---

## 🎉 Success!

You now have a fully functional, type-safe SOAP client in Rust!

**What's next?**
- Try different SOAP services
- Explore error handling
- Read the architecture docs
- Contribute to the project

---

**Happy SOAP-ing! 🧼✨**