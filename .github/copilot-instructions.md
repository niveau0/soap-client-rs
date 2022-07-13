# GitHub Copilot Instructions

> Instructions for GitHub Copilot when generating code for this project.
> See `.github/AI_ASSISTANTS.md` for an overview of AI assistant configuration files.

---

## 🎨 Code Style

### Naming Conventions
- **Structs/Enums**: `PascalCase` (e.g., `WsdlModel`, `SoapClient`)
- **Functions/Methods**: `snake_case` (e.g., `parse_wsdl`, `generate_code`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `SOAP_11_NAMESPACE`)
- **Type Parameters**: Single uppercase letter or `PascalCase` (e.g., `T`, `BufRead`)
- **Lifetimes**: Single lowercase letter (e.g., `'a`, `'de`)

### Generated Code Naming
- **Fields**: Convert XML `camelCase` to Rust `snake_case`
- **Types**: Convert XML names to Rust `PascalCase`
- **Enums**: Use descriptive names, not abbreviations

---

## 🔧 Rust Conventions

### Error Handling
- **Always use `Result<T, E>`** - Never panic in library code
- **Parser functions**: `Result<T, Box<dyn Error>>` for flexibility
- **Public API**: Use custom error types (e.g., `SoapError`, `CodegenError`)
- **Add context to errors**: Include what failed, not just that it failed
- **Use `?` operator** instead of `.unwrap()` or `.expect()`

### Async Code
- **Use `async fn`** for I/O operations
- **Use `tokio`** as the async runtime
- **Avoid `.await` in loops** when possible (use `futures::join!` or `try_join!`)
- **Document async behavior** in doc comments

### Memory & Performance
- **Prefer `&str` over `String`** for function parameters
- **Use `impl Trait`** for return types when appropriate
- **Clone only when necessary** - prefer borrowing
- **Use `HashMap` for lookups**, `Vec` for iteration
- **Avoid unnecessary allocations** in hot paths

---

## 📚 Documentation

### Doc Comments
- **All public items must have doc comments** (`///`)
- **Include examples** in doc comments for non-trivial functions
- **Document errors** with `# Errors` section
- **Document panics** with `# Panics` section (if any)
- **Use markdown** in doc comments (code blocks, lists, etc.)

### Example Format
```rust
/// Parse a WSDL XML string into a structured model.
///
/// # Arguments
///
/// * `xml` - The WSDL content as a string
///
/// # Returns
///
/// A `WsdlModel` containing all parsed definitions
///
/// # Errors
///
/// Returns an error if the XML is malformed or invalid WSDL
///
/// # Example
///
/// ```
/// let wsdl = r#"<definitions>...</definitions>"#;
/// let model = parse_wsdl(wsdl)?;
/// ```
pub fn parse_wsdl(xml: &str) -> Result<WsdlModel, Box<dyn Error>> {
    // ...
}
```

---

## 🏗️ Module Organization

### Visibility
- **Default to private** - Only make public what's needed
- **Use `pub(crate)`** for internal API within a crate
- **Use `pub(super)`** for internal API within a module
- **Keep internal types private** unless needed for public API

### Module Structure
- **Small, focused modules** (< 500 lines per file)
- **Group related functionality** (e.g., all parser functions in one module)
- **Separate public API from implementation**
- **Use sub-modules** for complex features (see `parser/wsdl/` and `parser/xsd/`)

---

## 🧪 Testing

### Test Organization
- **Unit tests** in same file with `#[cfg(test)] mod tests`
- **Integration tests** in `tests/` directory
- **Use descriptive test names** that explain what's being tested
- **Test both success and error cases**
- **Use `assert_eq!` with helpful messages**

### Test Patterns
```rust
#[test]
fn parses_valid_wsdl() {
    let wsdl = r#"<definitions>...</definitions>"#;
    let result = parse_wsdl(wsdl);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().services.len(), 1);
}

#[test]
fn rejects_invalid_wsdl() {
    let wsdl = r#"<invalid>..."#;
    let result = parse_wsdl(wsdl);
    assert!(result.is_err());
}
```

---

## 🎯 Type Design

### Prefer Owned Types for Public API
- **Public structs own their data** (use `String`, not `&str`)
- **Internal functions can borrow** (`&str`, `&[u8]`)
- **Use `Into<T>` for flexible parameters**

### Builder Pattern
- **Use for complex construction** (e.g., `SoapClient::builder()`)
- **Make builder methods chainable** (return `self`)
- **Add validation in `build()`** method

### Smart Derives
- **Always derive `Debug`** for structs/enums
- **Derive `Clone`** when data should be clonable
- **Derive `Default`** for types with sensible defaults
- **Derive `PartialEq, Eq`** for value types (avoid for floats)
- **Use `#[derive(Serialize, Deserialize)]`** for types that cross API boundaries

---

## 🔄 Serde Integration

### XML Serialization
- **Use `#[serde(rename = "...")]`** for XML element names
- **Handle namespaces** explicitly when needed
- **Use `quick-xml`** for parsing, `serde` for deserialization
- **Flatten nested structures** when appropriate

### Example
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Add {
    #[serde(rename = "intA")]
    pub int_a: i32,
    #[serde(rename = "intB")]
    pub int_b: i32,
}
```

---

## 🚫 What to Avoid

### Don't
- ❌ Use `.unwrap()` or `.expect()` in library code
- ❌ Use `panic!()` for recoverable errors
- ❌ Ignore compiler warnings
- ❌ Use `unsafe` without clear justification and documentation
- ❌ Clone unnecessarily
- ❌ Use `String` concatenation in loops (use `format!` or buffer)
- ❌ Return `Option<T>` when error details are needed (use `Result`)
- ❌ Use abbreviations in names (prefer `message` over `msg`)

### Be Careful With
- ⚠️ **Floating point comparisons** - Use epsilon or avoid `Eq` derive
- ⚠️ **Large XML documents** - Consider streaming parsers
- ⚠️ **Recursive structures** - Limit depth to avoid stack overflow
- ⚠️ **Resource cleanup** - Use RAII, avoid manual cleanup

---

## 🔐 Security

- **Validate all external input** (WSDL, XML, user data)
- **Limit resource consumption** (depth, size, iterations)
- **Use safe XML parsing** - Disable external entities
- **Sanitize identifiers** when generating code (escape Rust keywords)
- **Don't log sensitive data** (passwords, tokens)

---

## 🎨 Code Generation Specifics

### When Generating Rust Code from WSDL/XSD

- **Convert names to Rust conventions** (snake_case, PascalCase)
- **Escape Rust keywords** (`type` → `type_`, `self` → `self_`)
- **Add appropriate derives** based on field types
- **Generate doc comments** from WSDL documentation when available
- **Use `Option<T>` for `minOccurs="0"`**
- **Use `Vec<T>` for `maxOccurs="unbounded"`**
- **Combine for `Option<Vec<T>>`** when both apply

### Generated Code Quality
- **Should compile without warnings**
- **Should pass clippy** (run `cargo clippy` on generated code)
- **Should be readable** - prefer clarity over brevity
- **Should be idiomatic** - use Rust patterns, not XML patterns

---

## 📦 Dependencies

### When to Add Dependencies
- **Prefer standard library** when possible
- **Use well-maintained crates** (check downloads, recent commits)
- **Minimize dependencies** - each adds compile time
- **Check licenses** - must be MIT compatible

### Current Stack
- `quick-xml` - XML parsing
- `serde` - Serialization
- `reqwest` - HTTP client
- `tokio` - Async runtime (features: `rt-multi-thread`, `macros`)
- `thiserror` - Error types
- `log` - Logging facade

---

## 🔧 Project-Specific Patterns

### Parser Pattern
```rust
impl<B: std::io::BufRead> Parser<B> {
    pub(super) fn parse_element(&mut self, e: &BytesStart) -> Result<T, Box<dyn Error>> {
        // Extract attributes
        // Read child elements
        // Build and return result
    }
}
```

### Code Generator Pattern
```rust
pub fn generate_struct(complex_type: &ComplexType) -> String {
    let mut code = String::new();
    // Add derives
    // Add struct definition
    // Add fields
    code
}
```

### Runtime Client Pattern
```rust
impl SoapClient {
    pub async fn call<Req, Resp>(&self, request: &Req) -> SoapResult<Resp>
    where
        Req: Serialize,
        Resp: for<'de> Deserialize<'de>,
    {
        // Build envelope
        // Send HTTP request
        // Parse response
    }
}
```

---

**Remember**: The goal is idiomatic, safe, performant Rust code that's easy to understand and maintain.