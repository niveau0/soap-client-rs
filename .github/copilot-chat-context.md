# Copilot Chat Context - soapus

> **Purpose**: This file provides context for chat-based AI assistants across sessions.
> **Companion Files**: See `.github/copilot-instructions.md` for coding style and `.github/AI_ASSISTANTS.md` for overview.
> **Last Updated**: 2024-12-17
> **Project Status**: Phase 1 MVP Complete ✅ + Phase 2 Observability Complete ✅

---

## 🎯 Project Overview

**soapus** is a type-safe SOAP client generator for Rust that generates code from WSDL files at build time.

- **Language**: Rust (Edition 2021)
- **License**: MIT (single license, not dual)
- **Structure**: Cargo workspace with 3 main crates
- **Current Version**: 0.1.0 (pre-release)

---

## 📦 Workspace Structure

```
soapus/
├── soapus-codegen/     # Code generator (WSDL → Rust)
├── soapus-runtime/     # Runtime library (HTTP + SOAP)
├── soapus-cli/         # CLI tool for WSDL parsing & code generation
├── examples/calculator/     # Working example
├── examples/observability/  # Observability example (OpenTelemetry/Prometheus)
└── testdata/wsdl/          # Shared test WSDL files
```

### Key Directories

- **`testdata/wsdl/`** - Shared WSDL files for testing
  - `calculator.wsdl` - Simple calculator service
  - `countryinfo.wsdl` - Complex service with many operations
  - `numberconversion.wsdl` - Number to words conversion
- **`docs/`** - Additional documentation
  - `TRACING.md` - Tracing and observability guide

---

## 🏗️ Architecture Decisions

### Parser Organization (Recent Refactoring ✅)

Both WSDL and XSD parsers follow the same modular structure:

**WSDL Parser** (`soapus-codegen/src/parser/wsdl/`):
```
wsdl/
├── mod.rs              # Models & helper methods
├── parser.rs           # Orchestration (318 lines)
├── definitions.rs      # Root element attributes
├── types.rs            # XSD schema extraction
├── message.rs          # Message definitions
├── port_type.rs        # PortType & operations
├── binding.rs          # SOAP binding details
└── service.rs          # Service endpoints
```

**XSD Parser** (`soapus-codegen/src/parser/xsd/`):
```
xsd/
├── mod.rs              # Models
├── parser.rs           # Orchestration (179 lines)
├── schema_attributes.rs # Schema attributes
├── schema_content.rs   # Top-level elements
├── element.rs          # Element definitions
├── complex_type.rs     # ComplexType definitions
├── sequence.rs         # Sequence & all compositors
└── simple_type.rs      # SimpleType & utilities
```

**Pattern**: 
- `parser.rs` = Orchestration only
- Sub-modules = Specific parsing logic
- `pub(super)` for internal APIs
- Tests in `parser.rs`

### Dependency Management

**Tokio Features** (Optimized ✅):
- Using: `rt-multi-thread`, `macros`
- NOT using: `full` (saves ~15-20% compile time)
- Reason: `reqwest` brings its own tokio features

**Tracing Features** (Phase 1 ✅):
- `tracing` - Default enabled for structured logging
- `tracing-subscriber` - Only in examples/dev-dependencies
- OpenTelemetry/Metrics - Planned for Phase 2 (optional features)

**Dependencies**:
- `quick-xml` - XML parsing
- `serde` - Serialization
- `reqwest` - HTTP client
- `tokio` - Async runtime
- `tracing` - Structured logging (optional, default ON)
- No proc-macros (build.rs approach preferred)

---

## 🎨 Code Style & Conventions

### Naming
- **Structs/Enums**: `PascalCase`
- **Functions**: `snake_case`
- **Constants**: `UPPER_SNAKE_CASE`
- **Generated Fields**: `snake_case` (converted from XML camelCase)
- **Generated Types**: `PascalCase`

### Error Handling
- Parser: `Result<T, Box<dyn Error>>` (flexibility)
- Generator: `Result<T, CodegenError>` (typed)
- Runtime: `SoapResult<T>` = `Result<T, SoapError>`

### Visibility
- `pub(super)` - For parser sub-modules
- `pub` - Public API only
- Private by default

---

## ✅ What Works (Phase 1 Complete)

### Core Features
- ✅ WSDL 1.1 parsing (complete)
- ✅ XSD schema parsing (complexType, simpleType, sequences)
- ✅ Rust code generation (structs, enums, async methods)
- ✅ SOAP 1.1 & 1.2 support
- ✅ HTTP client with reqwest
- ✅ Namespace handling
- ✅ SOAPAction support
- ✅ SOAP fault detection
- ✅ Build-time generation via build.rs
- ✅ Tracing integration (Phase 1)
  - ✅ Structured logging with `tracing` crate
  - ✅ Spans in runtime (HTTP requests, SOAP operations)
  - ✅ Events in parser/codegen (progress, errors)
  - ✅ Generated code includes tracing spans
  - ✅ Configurable via RUST_LOG environment variable

### Testing
- ✅ 57 tests passing (35 unit + 4 integration + 18 runtime)
- ✅ Working calculator example
- ✅ Working observability example (OpenTelemetry/Prometheus)
- ✅ CLI tool (parse, generate, info commands)
- ✅ Tested with 3 real WSDL files

### Documentation
- ✅ README.md - Overview
- ✅ QUICKSTART.md - 5-minute guide
- ✅ TODO.md - Current tasks and roadmap

---

## 🚧 Current Focus (Next Steps)

### Priority 1: Production Readiness
- ✅ Tracing/Logging integration (Phase 1 complete)
- [ ] OpenTelemetry/Jaeger export (Phase 2)
- [ ] Metrics collection (Prometheus) (Phase 2)
- [ ] Testing with more real-world WSDLs
- [ ] Performance benchmarking

### Priority 2: HTTP Features
- [ ] HTTP compression (gzip, deflate)
- [ ] Retry logic with exponential backoff
- [ ] Cookie/Session handling
- [ ] Custom HTTP headers API

### Priority 3: Advanced Features
- [ ] WS-Security (UsernameToken)
- [ ] MTOM/XOP (binary attachments)
- [ ] WSDL imports/includes

See `TODO.md` for complete list.

---

## 🔧 Development Workflow

### Building
```bash
cargo build --all
cargo test --all
cargo check --all
```

### Running Example
```bash
cd examples/calculator
cargo run
```

### Testing with Specific WSDL
```rust
// Unit test pattern
let wsdl = include_str!("../../../../testdata/wsdl/calculator.wsdl");
let model = parse_wsdl(wsdl).unwrap();
```

---

## 📋 Important Constraints & Patterns

### What We Support
- ✅ Document/literal SOAP style
- ✅ WSDL 1.1
- ✅ SOAP 1.1 & 1.2
- ✅ Basic XSD types (all built-ins mapped)
- ✅ ComplexType with sequences
- ✅ SimpleType with enumerations
- ✅ minOccurs/maxOccurs → Option<T>/Vec<T>

### What We DON'T Support (Yet)
- ❌ RPC/encoded style
- ❌ WSDL 2.0
- ❌ XSD choice/all (partial)
- ❌ Type inheritance (extension/restriction)
- ❌ WS-Security
- ❌ MTOM/XOP

### Design Philosophy
1. **Type Safety First** - Generate Rust types, not strings
2. **Build-Time Generation** - Not proc-macros (better IDE support)
3. **Simple API** - Easy to use, hard to misuse
4. **Standard Compliance** - Follow W3C specs
5. **Real-World Focus** - Works with actual SOAP services

---

## 🐛 Known Issues & Quirks

### None Currently! 🎉
All tests passing, example working perfectly.

### Future Considerations
- XSD `choice` not fully implemented (rarely used in SOAP)
- SimpleType restrictions partially implemented (enums work)
- No WSDL 2.0 support (only 1.1)

---

## 📝 Common Tasks

### Adding a New Parser Module
1. Create `module_name.rs` in appropriate parser directory
2. Add `mod module_name;` to `mod.rs`
3. Implement `impl ParserStruct { pub(super) fn parse_X(...) }`
4. Add tests in `parser.rs` or separate test file

### Adding a New Feature
1. Update TODO.md with task
2. Implement in appropriate crate
3. Add tests (aim for >80% coverage)
4. Update relevant documentation
5. Test with real WSDL files

### Debugging Generated Code
```bash
# Find generated code location
cargo build -p calculator-example

# View generated code
cat target/debug/build/*/out/soap_client.rs
```

---

## 🎓 Learning Resources

### SOAP/WSDL Specs
- [WSDL 1.1 Spec](https://www.w3.org/TR/wsdl)
- [SOAP 1.1](https://www.w3.org/TR/2000/NOTE-SOAP-20000508/)
- [SOAP 1.2](https://www.w3.org/TR/soap12/)
- [XML Schema Primer](https://www.w3.org/TR/xmlschema-0/)

### Key Files to Understand
1. `soapus-codegen/src/parser/wsdl/parser.rs` - WSDL parsing entry
2. `soapus-codegen/src/generator/rust_codegen.rs` - Code generation
3. `soapus-runtime/src/client.rs` - HTTP client
4. `soapus-runtime/src/envelope.rs` - SOAP envelope building
5. `examples/calculator/build.rs` - Usage example

---

## 💡 Quick Wins for New Contributors

1. **Test with your WSDL** - Try generating from your SOAP service
2. **Add example** - Create example for different service
3. **Documentation** - Improve guides, add tutorials
4. **Error messages** - Make them more helpful
5. **Logging** - Add tracing throughout codebase

---

## 🔍 Debugging Tips

### Parser Issues
```bash
RUST_LOG=debug cargo test test_name
```

### Generated Code Issues
1. Check `target/debug/build/*/out/soap_client.rs`
2. Look for compilation errors
3. Check type mappings in `generator/type_mapper.rs`

### Runtime Issues
1. Enable request/response logging
2. Check SOAPAction header
3. Verify namespace URIs
4. Test with curl/Postman first

---

## 📞 Project Contacts

- **Repository**: (to be published to crates.io)
- **Issues**: GitHub Issues (when published)
- **Discussions**: GitHub Discussions (when published)

---

## 🔄 Recent Changes

### 2024-12-17 (Part 3 - CLI Tool & CI)
- ✅ **CLI Tool Implementation**
  - ✅ Created `soapus-cli` package
  - ✅ Commands: `parse`, `generate`, `info`
  - ✅ Options: verbose, output dir, client name, SOAP version
- ✅ **CI/CD Pipeline**
  - ✅ GitHub Actions workflow
  - ✅ Tests, clippy, rustfmt checks
  - ✅ Example builds (calculator, observability, CLI)
  - ✅ Rust cache optimization with `Swatinem/rust-cache@v2`
- ✅ **Repository Cleanup**
  - ✅ Fixed WSDL paths in examples (use testdata/)
  - ✅ Updated all GitHub URLs to `niveau0/soapus`
  - ✅ Created `examples/README.md` with IDE setup guide
  - ✅ Updated TODO.md to reflect CLI completion

### 2024-12-17 (Part 2 - Tracing Integration)
- ✅ **Phase 1 Tracing Implementation**
  - ✅ Replaced `log` crate with `tracing` throughout codebase
  - ✅ Added tracing spans to runtime (SoapClient, envelope building)
  - ✅ Added logging events to parser (WSDL/XSD parsing progress)
  - ✅ Added logging events to codegen (generation milestones)
  - ✅ Generator produces code with tracing spans
  - ✅ Features: `tracing` (default ON), `metrics` (opt-in for Phase 2)
  - ✅ Updated calculator example with tracing-subscriber
  - ✅ Created `docs/TRACING.md` documentation
  - ✅ All tests passing (57 tests)

### 2024-12-17 (Part 1 - Refactoring)
- ✅ Refactored WSDL parser into 7 modules (65% smaller main file)
- ✅ Refactored XSD parser into 7 modules (62% smaller main file)
- ✅ Optimized tokio features (removed "full", using specific features)
- ✅ Moved test WSDLs to `testdata/wsdl/` directory
- ✅ Updated to MIT license only (was MIT OR Apache-2.0)
- ✅ Created AI assistant context files

### Earlier (Phase 1 - MVP)
- ✅ Complete WSDL/XSD parsing
- ✅ Full code generation
- ✅ SOAP runtime implementation
- ✅ Working calculator example
- ✅ Comprehensive documentation

---

## 🎯 Success Metrics

- **Tests**: 57/57 passing ✅
- **Examples**: 2/2 working (calculator + observability) ✅
- **CLI Tool**: Fully functional (parse, generate, info) ✅
- **Docs**: All major docs complete + TRACING.md + examples/README.md ✅
- **Real WSDL Compatibility**: 3/3 tested services ✅
- **Code Quality**: No clippy warnings ✅
- **Observability**: Phase 1 tracing + Phase 2 OpenTelemetry/Prometheus ✅
- **CI/CD**: GitHub Actions pipeline operational ✅

---

---

## 🔍 Tracing & Observability

### Current State (Phase 1 Complete)
- **Tracing**: Integrated throughout runtime, parser, and codegen
- **Log Levels**: ERROR (internal bugs), WARN (external failures), INFO (milestones), DEBUG (details)
- **Spans**: Hierarchical tracing in runtime operations
- **Generated Code**: Automatically includes tracing spans
- **Configuration**: Via RUST_LOG environment variable

### Example Usage
```bash
# Run with info logging
RUST_LOG=info cargo run

# Debug level for detailed output
RUST_LOG=debug cargo run

# Filter by module
RUST_LOG=soapus_runtime=debug cargo run
```

### Next Steps (Phase 2)
- OpenTelemetry/Jaeger integration (optional feature)
- Metrics collection with Prometheus export (optional feature)
- Create `examples/observability/` with full setup

See `docs/TRACING.md` for complete guide.

---

**For new chat sessions**: Read this file first to understand current state, decisions, and conventions.