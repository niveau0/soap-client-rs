# Calculator SOAP Client Example

This example demonstrates how to use the SOAP client generator to create a type-safe client for a real public SOAP web service.

## Overview

The Calculator service (http://www.dneonline.com/calculator.asmx) provides basic arithmetic operations:
- **Add** - Addition of two integers
- **Subtract** - Subtraction of two integers
- **Multiply** - Multiplication of two integers
- **Divide** - Division of two integers

This example shows how to:
1. Generate a SOAP client from a WSDL file at build time
2. Use the generated client to make SOAP requests
3. Handle SOAP responses and errors
4. Work with a real, live SOAP service (no credentials needed!)

## Structure

```
calculator/
├── Cargo.toml          # Project dependencies
├── build.rs            # Build script that generates the client
└── src/
    └── main.rs         # Example usage with all 4 operations

WSDL file location: ../../testdata/wsdl/calculator.wsdl
```

## How It Works

### Build Time Code Generation

The `build.rs` script runs during compilation and generates Rust code from the WSDL:

```rust
use soap_client_codegen::SoapClientGenerator;
use std::path::PathBuf;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let wsdl_path = PathBuf::from(&manifest_dir).join("../../testdata/wsdl/calculator.wsdl");
    
    println!("cargo:rerun-if-changed={}", wsdl_path.display());
    
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    
    SoapClientGenerator::builder()
        .wsdl_path(wsdl_path.to_str().expect("Invalid WSDL path"))
        .out_dir(&out_dir)
        .generate()
        .expect("Failed to generate SOAP client from WSDL");
}
```

This creates a complete, type-safe SOAP client with:
- Rust structs for all XSD types
- Async methods for all SOAP operations
- Automatic serialization/deserialization
- SOAP envelope handling
- Namespace and SOAPAction support

### Runtime Usage

The generated client is included in `main.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));

#[tokio::main]
async fn main() -> SoapResult<()> {
    let calculator = Calculator::new("http://www.dneonline.com/calculator.asmx");
    
    // Make SOAP calls
    let result = calculator.add(Add { int_a: 5, int_b: 3 }).await?;
    println!("5 + 3 = {}", result.add_result);
    
    Ok(())
}
```

## Building

```bash
cargo build
```

The build process will:
1. Run `build.rs` to parse `service.wsdl`
2. Generate Rust code in `$OUT_DIR/soap_client.rs`
3. Compile the example with the generated code

## Running

```bash
cargo run
```

**Expected output:**
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

## Generated Types

The code generator creates Rust types for all XSD definitions:

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
```

Notice:
- Fields are converted to `snake_case` for Rust conventions
- `#[serde(rename = "...")]` preserves the XML element names
- Derives are optimized based on content (PartialEq, Eq when appropriate)

## Generated Operations

All WSDL operations become async methods:

```rust
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
    
    /// Calls the Subtract operation
    pub async fn subtract(&self, request: Subtract) -> SoapResult<SubtractResponse> { ... }
    
    /// Calls the Multiply operation
    pub async fn multiply(&self, request: Multiply) -> SoapResult<MultiplyResponse> { ... }
    
    /// Calls the Divide operation
    pub async fn divide(&self, request: Divide) -> SoapResult<DivideResponse> { ... }
}
```

## Features Demonstrated

- ✅ Build-time WSDL parsing and code generation
- ✅ Type-safe SOAP client with async/await
- ✅ Automatic XML serialization/deserialization with serde
- ✅ SOAP 1.1 envelope handling
- ✅ SOAPAction header support
- ✅ XML namespace handling
- ✅ Error handling with custom error types
- ✅ Real SOAP calls to a live service
- ✅ Support for simple types (integers)
- ✅ Idiomatic Rust code generation

## Inspecting Generated Code

You can view the generated code after building:

```bash
# On Unix/Linux/Mac
find target/debug/build -name "soap_client.rs" -exec cat {} \;

# On Windows
dir target\debug\build\*\out\soap_client.rs /s /b
```

This is useful for:
- Understanding what was generated
- Debugging issues
- Learning how the generator works

## Troubleshooting

### Build fails with "WSDL file not found"
This should not happen as the WSDL is in the shared `testdata/wsdl/` directory. If it does, ensure the repository is fully cloned.

### Runtime error: "Connection refused"
The public SOAP service might be down. Try again later or check if the URL is still valid.

### Generated code doesn't compile
This shouldn't happen with this example. If it does, please file an issue!

## Extending This Example

### Try Different Values

```rust
// Test edge cases
let result = calculator.divide(Divide { int_a: 10, int_b: 0 }).await;
// Will return a SOAP fault for division by zero

// Test negative numbers
let result = calculator.subtract(Subtract { int_a: 5, int_b: 10 }).await?;
println!("Result: {}", result.subtract_result); // -5
```

### Add Error Handling

```rust
match calculator.divide(request).await {
    Ok(response) => {
        println!("Result: {}", response.divide_result);
    }
    Err(SoapError::SoapFault { fault_code, fault_string }) => {
        eprintln!("SOAP Fault: {} - {}", fault_code, fault_string);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

### Use with Your Own WSDL

1. Place your WSDL file in your project directory
2. Update `build.rs` to point to your WSDL file
3. Run `cargo build`
4. The generator will create a client for your service!

## What's Next?

- Check out the [main README](../../README.md) for project overview
- Read the [Quick Start Guide](../../QUICKSTART.md) to create your own client
## License

This example is part of the soapus project and is licensed under the same terms (MIT).

## Credits

- Public SOAP service provided by: http://www.dneonline.com
- Built with: soapus, tokio, reqwest, serde, quick-xml