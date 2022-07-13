# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- WS-Security support (UsernameToken)
- MTOM/XOP binary attachments
- HTTP compression (gzip, deflate)
- Retry logic with exponential backoff
- Cookie/Session handling
- WSDL imports/includes

---

## [0.1.0] - 2024-12-XX

> **Note**: This is the initial pre-release version. The project was developed with significant AI assistance and has undergone manual review and testing. Please conduct thorough testing before using in production.

### 🎉 Initial Release

#### Core Features

**WSDL & XSD Parsing**
- Complete WSDL 1.1 parsing (definitions, types, messages, portTypes, bindings, services)
- XML Schema (XSD) parsing with ComplexType, SimpleType, sequences
- Automatic namespace handling
- SOAPAction and endpoint URL extraction
- Embedded schema extraction
- minOccurs/maxOccurs → `Option<T>` / `Vec<T>` mapping

**Code Generation**
- Type-safe Rust structs from XSD types
- Enums from SimpleType restrictions
- Async operation methods for each WSDL operation
- Automatic serde serialization attributes
- Smart derives (`Debug`, `Clone`, `PartialEq`, `Default`)
- Rust naming conventions (snake_case fields, PascalCase types)
- Keyword sanitization (e.g., `type` → `type_`)
- Build-time generation via `build.rs`

**SOAP Runtime**
- SOAP 1.1 and 1.2 envelope building
- Async HTTP client based on `reqwest` and `tokio`
- Automatic XML serialization/deserialization with `serde`
- SOAP fault detection and parsing
- Configurable timeouts
- Custom HTTP client support via builder pattern

**Observability**
- Structured logging with `tracing` crate (default feature)
- OpenTelemetry/Jaeger distributed tracing (optional `opentelemetry` feature)
- Prometheus metrics export (optional `metrics` feature)
- JSON logging support
- Tracing spans automatically added to generated client methods

**CLI Tool** (`soapus-cli`)
- `parse` - Parse and validate WSDL files
- `info` - Display WSDL information (services, operations, types)
- `generate` - Generate Rust client code from WSDL
- Options for client name, SOAP version, output directory

#### Documentation

- Comprehensive README with quick start guide
- `QUICKSTART.md` - Get started in 5 minutes
- `docs/TRACING.md` - Observability and logging guide
- `docs/API_DOCUMENTATION.md` - Runtime API reference
- Working examples:
  - `examples/calculator` - Basic SOAP client usage
  - `examples/observability` - Full observability stack with Jaeger/Prometheus/Grafana
- AI assistant configuration (`.github/copilot-instructions.md`, `.github/copilot-chat-context.md`)

#### Testing

- 57 comprehensive tests
  - 35 unit tests (parser and generator)
  - 4 integration tests (end-to-end code generation)
  - 18 runtime tests (envelope building, client, fault handling)
- Tested with real-world WSDL files:
  - Calculator service (http://www.dneonline.com/calculator.asmx)
  - Country information service
  - Number conversion service

#### Crate Structure

- **`soapus-codegen`** - WSDL/XSD parser and Rust code generator
- **`soapus-runtime`** - HTTP client and SOAP envelope building
- **`soapus-cli`** - Command-line tool for WSDL processing

#### Dependencies

- `quick-xml` - Fast XML parsing
- `serde` - Serialization framework
- `reqwest` - Async HTTP client
- `tokio` - Async runtime
- `tracing` - Structured logging (optional, default ON)
- `thiserror` - Error types

### What Works

✅ Document/literal SOAP style  
✅ WSDL 1.1  
✅ SOAP 1.1 & 1.2  
✅ All XSD built-in types  
✅ ComplexType with sequences  
✅ SimpleType with enumerations  
✅ Async/await  
✅ Namespace handling  
✅ SOAP fault detection  
✅ Distributed tracing  
✅ Metrics collection  

### Known Limitations

❌ RPC/encoded style not supported  
❌ WSDL 2.0 not supported (only 1.1)  
❌ XSD choice/all partially implemented  
❌ Type inheritance (extension/restriction) limited  
❌ No WS-Security yet  
❌ No MTOM/XOP yet  

### Migration Notes

This is the first release - no migration needed.

---

## Project Roadmap

### Phase 1 - MVP ✅ (Complete)
- Core WSDL/XSD parsing
- Code generation
- SOAP runtime
- Basic tracing

### Phase 2 - Observability ✅ (Complete)
- OpenTelemetry integration
- Prometheus metrics
- JSON logging
- Full observability example

### Phase 3 - Production Readiness 🚧 (In Progress)
- WS-Security
- Enhanced error messages
- More real-world WSDL testing
- Performance benchmarks

### Phase 4 - Advanced Features 📋 (Planned)
- MTOM/XOP attachments
- HTTP compression & retry
- Cookie/session handling
- WSDL imports

---

[Unreleased]: https://github.com/niveau0/soapus/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/niveau0/soapus/releases/tag/v0.1.0