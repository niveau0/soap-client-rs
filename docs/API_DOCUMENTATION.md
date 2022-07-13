# API Documentation

This document describes how API documentation works in `soapus`, including automatically extracted WSDL documentation and runtime API documentation.

---

## 🎯 Overview

The SOAP client generator provides comprehensive API documentation through:

1. **WSDL Documentation Extraction** - Automatically extracts `<wsdl:documentation>` elements from WSDL files
2. **Runtime API Documentation** - Fully documented public API with examples
3. **Generated Code Documentation** - Type-safe client code with descriptive doc comments

---

## 📚 WSDL Documentation Extraction

### How It Works

When your WSDL file contains `<wsdl:documentation>` elements, the generator automatically extracts them and includes them as Rust doc comments in the generated code.

### Example WSDL with Documentation

```xml
<wsdl:portType name="CalculatorSoap">
  <wsdl:operation name="Add">
    <wsdl:documentation>
      Adds two integers. This is a test WebService.
    </wsdl:documentation>
    <wsdl:input message="tns:AddSoapIn" />
    <wsdl:output message="tns:AddSoapOut" />
  </wsdl:operation>
</wsdl:portType>
```

### Generated Rust Code

```rust
impl Calculator {
    /// Call the Add operation
    ///
    /// Adds two integers. This is a test WebService.
    ///
    /// # Arguments
    /// * `request` - The Add request
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, request)))]
    pub async fn add(&self, request: Add) -> SoapResult<AddResponse> {
        self.client.call_with_soap_action("Add", Some("http://tempuri.org/Add"), Some(TARGET_NAMESPACE), &request).await
    }
}
```

### Documentation Structure

The generated documentation includes:

1. **Operation Name** - "Call the [Operation] operation"
2. **WSDL Documentation** - Extracted from `<wsdl:documentation>` (if present)
3. **Parameters Section** - Describes the request type
4. **Tracing Attributes** - For observability (if `tracing` feature enabled)

### Supported Documentation Locations

Currently, documentation is extracted from:

- ✅ **Operation-level documentation** (`<wsdl:operation>`)
- 🚧 **Type-level documentation** (planned for future release)
- 🚧 **Field-level documentation** (planned for future release)

---

## 🔧 Runtime API Documentation

All public APIs in `soapus-runtime` are fully documented with examples.

### `SoapClient` - Main Client

The core client for making SOAP requests:

```rust
use soapus_runtime::SoapClient;

let client = SoapClient::new("http://example.com/soap");
```

**Key Methods:**

- `new(endpoint)` - Create a new client with default settings
- `builder(endpoint)` - Create a builder for advanced configuration
- `call(operation, request)` - Call a SOAP operation
- `call_with_soap_action(...)` - Call with custom SOAPAction header
- `endpoint()` - Get the endpoint URL
- `set_soap_version(version)` - Set SOAP 1.1 or 1.2

### `SoapClientBuilder` - Advanced Configuration

For customizing client behavior:

```rust
use soapus_runtime::{SoapClient, SoapVersion};
use std::time::Duration;

let client = SoapClient::builder("http://example.com/soap")
    .soap_version(SoapVersion::Soap12)
    .timeout(Duration::from_secs(60))
    .soap_action("http://example.com/MyOperation")
    .build();
```

**Builder Methods:**

- `soap_version(version)` - Set SOAP protocol version (1.1 or 1.2)
- `soap_action(action)` - Set default SOAPAction header
- `timeout(duration)` - Set request timeout
- `http_client(client)` - Provide custom reqwest Client
- `build()` - Construct the configured client

### `SoapVersion` - Protocol Version

```rust
use soapus_runtime::SoapVersion;

// SOAP 1.1 (default)
let version = SoapVersion::Soap11;

// SOAP 1.2
let version = SoapVersion::Soap12;
```

### `SoapError` - Error Types

Comprehensive error handling:

```rust
use soapus_runtime::SoapError;

match client.call("MyOp", &request).await {
    Ok(response) => { /* success */ },
    Err(SoapError::HttpError(e)) => { /* HTTP transport error */ },
    Err(SoapError::SoapFault { code, message, detail }) => {
        eprintln!("SOAP Fault: {} - {}", code, message);
    },
    Err(SoapError::XmlError(e)) => { /* XML parsing error */ },
    Err(SoapError::DeserializationError(e)) => { /* Type conversion error */ },
    Err(e) => { /* Other errors */ },
}
```

**Error Variants:**

- `HttpError` - HTTP request/response errors (from reqwest)
- `XmlError` - XML parsing errors
- `SoapFault` - SOAP fault from server (with code, message, detail)
- `SerializationError` - Failed to serialize request
- `DeserializationError` - Failed to deserialize response
- `InvalidResponse` - Malformed SOAP response
- `MissingField` - Required field not present
- `InvalidConfig` - Invalid client configuration
- `Other` - Other errors

### `SoapResult<T>` - Result Type Alias

Convenient type alias for SOAP operations:

```rust
pub type SoapResult<T> = Result<T, SoapError>;

// Used in generated code:
pub async fn my_operation(&self, request: MyRequest) -> SoapResult<MyResponse> {
    // ...
}
```

---

## 📖 Viewing Generated Documentation

### Using `cargo doc`

Generate and view documentation for your project:

```bash
# Generate documentation
cargo doc --open

# Include private items (useful during development)
cargo doc --document-private-items --open

# Generate for specific package
cargo doc -p soapus-runtime --open
```

### Documentation in IDEs

Most Rust IDEs (rust-analyzer, IntelliJ IDEA, VS Code) show documentation on hover:

1. **Hover over a type/function** - Shows doc comments
2. **Auto-completion** - Displays documentation inline
3. **Quick documentation** - Press Ctrl+Q (IntelliJ) or Ctrl+K Ctrl+I (VS Code)

---

## ✍️ Best Practices

### For WSDL Authors

If you control your WSDL files, add documentation to improve generated code:

```xml
<wsdl:operation name="CreateUser">
  <wsdl:documentation>
    Creates a new user account in the system.
    
    The username must be unique and between 3-20 characters.
    Password must meet complexity requirements (min 8 chars, uppercase, lowercase, number).
    
    Returns the newly created user ID on success.
    Throws UserAlreadyExists fault if username is taken.
  </wsdl:documentation>
  <wsdl:input message="tns:CreateUserRequest"/>
  <wsdl:output message="tns:CreateUserResponse"/>
  <wsdl:fault name="UserAlreadyExists" message="tns:UserAlreadyExistsFault"/>
</wsdl:operation>
```

### For Library Users

1. **Always check generated documentation** - Run `cargo doc` after generation
2. **Use type-safe APIs** - Leverage Rust's type system
3. **Handle all error cases** - Match on specific `SoapError` variants
4. **Enable tracing** - Use the `tracing` feature for better observability

### For Code Reviewers

1. **Review generated documentation** - Ensure WSDL docs are meaningful
2. **Check for unclear naming** - Complex XML names may need manual clarification
3. **Verify error handling** - Confirm all SOAP faults are documented

---

## 🔮 Future Enhancements

Planned documentation improvements:

- [ ] Extract `<xs:documentation>` from XSD types
- [ ] Document struct fields from XSD annotations
- [ ] Include WSDL constraints in doc comments (e.g., minOccurs, maxOccurs)
- [ ] Generate usage examples from WSDL samples
- [ ] Support custom documentation templates
- [ ] Markdown formatting in WSDL documentation
- [ ] Multi-language documentation extraction

---

## 📚 Resources

- **Rust Documentation Guidelines**: https://doc.rust-lang.org/rustdoc/
- **WSDL Specification**: https://www.w3.org/TR/wsdl
- **XML Schema Documentation**: https://www.w3.org/TR/xmlschema-1/#Schemas

---

## 💡 Examples

### Minimal Example

```rust
// build.rs
use soapus_codegen::SoapClientGenerator;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    
    SoapClientGenerator::builder()
        .wsdl_path("service.wsdl")
        .out_dir(&out_dir)
        .generate()
        .expect("Failed to generate SOAP client");
}
```

```rust
// main.rs
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));

#[tokio::main]
async fn main() -> SoapResult<()> {
    // Generated client includes WSDL documentation!
    let client = MyService::new("http://example.com/soap");
    
    // Hover over 'my_operation' in your IDE to see documentation
    let response = client.my_operation(request).await?;
    
    Ok(())
}
```

### With Custom Error Handling

```rust
use soap_client_runtime::{SoapError, SoapResult};

async fn call_with_retry(client: &Calculator, request: Add) -> SoapResult<AddResponse> {
    for attempt in 1..=3 {
        match client.add(request.clone()).await {
            Ok(response) => return Ok(response),
            Err(SoapError::HttpError(e)) if attempt < 3 => {
                eprintln!("Attempt {} failed: {}, retrying...", attempt, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

---

**Enjoy well-documented, type-safe SOAP clients!** 🎉