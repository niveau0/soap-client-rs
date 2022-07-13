# TODO - Next Steps

## 🎉 MVP Complete!

**All core functionality is implemented and working!**

See `examples/calculator` for a fully functional SOAP client making real API calls.

---

## ✅ Completed (Phase 1 - MVP)

### WSDL Parser ✅
- [x] Parse WSDL definitions, types, messages, portTypes, bindings, services
- [x] Extract SOAPAction from bindings
- [x] Extract endpoint URLs from services
- [x] Extract embedded XSD schema from `<types>`
- [x] Comprehensive helper methods on WsdlModel
- [x] Full test coverage (33 tests)

### XSD Parser ✅
- [x] Parse complexType with sequences
- [x] Parse simpleType with restrictions/enumerations
- [x] Parse top-level elements
- [x] Handle minOccurs/maxOccurs (Option, Vec)
- [x] Handle nillable attributes
- [x] Empty element handling
- [x] Full test coverage (14 tests)

### Code Generator ✅
- [x] `rust_codegen.rs` fully implemented
- [x] Generate Rust structs from complexTypes
- [x] Generate Rust enums from simpleTypes
- [x] Generate async operation methods
- [x] Smart derives (Default, PartialEq, Eq)
- [x] Serde rename attributes
- [x] snake_case fields, PascalCase types
- [x] TARGET_NAMESPACE constant
- [x] Full test coverage (7 tests)

### SOAP Runtime ✅
- [x] `envelope.rs` - SOAP 1.1 and 1.2 envelope building
- [x] `client.rs` - Full async HTTP client
- [x] XML serialization with namespace support
- [x] Response parsing and deserialization
- [x] SOAP fault detection and parsing
- [x] Builder pattern for configuration
- [x] Timeout support
- [x] Custom HTTP client support
- [x] Full test coverage (14 tests + 3 doc tests)

### Example Projects ✅
- [x] `examples/calculator` - Complete working example
  - [x] Build-time code generation via build.rs
  - [x] Real SOAP calls to http://www.dneonline.com/calculator.asmx
  - [x] All 4 operations tested (Add, Subtract, Multiply, Divide)
  - [x] Proper error handling
  - [x] Documentation
- [x] `examples/observability` - Full observability stack
  - [x] OpenTelemetry/Jaeger integration
  - [x] Prometheus metrics
  - [x] Docker Compose setup
  - [x] Axum web server with metrics endpoint

### CLI Tool ✅
- [x] `soapus-cli` - Standalone command-line tool
  - [x] `parse` command - Validate WSDL files
  - [x] `generate` command - Generate Rust code from WSDL
  - [x] `info` command - Display WSDL information (services, operations, types)
  - [x] Verbose output option
  - [x] Configurable output directory
  - [x] Custom client name option
  - [x] SOAP version selection (1.1/1.2)

### Documentation ✅
- [x] README.md - Complete overview
- [x] QUICKSTART.md - 5-minute guide
- [x] TODO.md - Current tasks and roadmap (this file)

---

## 🎯 Priority 1: Production Readiness (Current Focus)

### Error Handling Improvements
- [ ] More descriptive error messages
- [ ] Include WSDL file name in errors
- [ ] Line numbers for parsing errors
- [ ] Suggestions for common mistakes
- [ ] Validation before code generation

### Code Quality
- [ ] Complete clippy warnings cleanup
- [ ] Add more inline documentation
- [ ] API documentation (docs.rs ready)
- [ ] Examples in doc comments
- [ ] CHANGELOG.md

### CI/CD
- [x] GitHub Actions workflow
  - [x] Run tests on push
  - [x] Clippy checks
  - [x] Format checks
  - [x] Build examples (calculator, observability, CLI)
  - [x] Verify generated code compiles
- [ ] Code coverage reporting
- [ ] Automated releases

**Examples Testing Strategy:**
- ✅ **calculator**: Build only (verifies code generation works)
- ✅ **observability**: Build only (no Docker required in CI)
- ✅ **soapus-cli**: Build and verify compilation
- ❌ **No runtime tests**: Examples require external services/Docker
  - Runtime testing happens manually or in local development
  - CI focuses on compilation and code generation verification

### Logging & Observability ✅
- [x] Integrate `tracing` crate for structured logging
- [x] Add span instrumentation to runtime operations
  - [x] HTTP requests/responses
  - [x] SOAP envelope building
  - [x] Response parsing
- [x] Add logging events to parser and codegen
  - [x] WSDL parsing progress
  - [x] Code generation milestones
- [x] Configurable log levels via RUST_LOG
- [x] Example with tracing subscriber setup (calculator)
- [x] Generated code includes tracing spans
- [ ] Request/response payload logging (sanitized) - Phase 2
- [ ] OpenTelemetry/Jaeger export - Phase 2
- [ ] Metrics export (Prometheus) - Phase 2

### Publishing
- [ ] Prepare for crates.io
  - [ ] Choose final crate names
  - [x] Add license files (MIT)
  - [ ] Add repository links
  - [ ] Add keywords and categories
- [ ] Publish v0.1.0
- [ ] Announce on Reddit, Discord, etc.

---

## 🎯 Priority 3: Advanced Features (4-8 Weeks)

### Observability Enhancements (Phase 2) ✅
- [x] OpenTelemetry integration
  - [x] Jaeger exporter for distributed tracing
  - [x] Span context propagation
  - [x] Example with docker-compose (Jaeger + service)
- [x] Metrics collection
  - [x] Request duration histogram
  - [x] Request counter (by operation, status)
  - [x] Error counter (by type)
  - [x] Prometheus exporter
  - [x] Example metrics endpoint
- [x] Create `examples/observability/` with full setup
  - [x] Axum web server with SOAP client
  - [x] `/metrics` endpoint for Prometheus
  - [x] Docker Compose with Jaeger + Prometheus + Grafana
  - [x] Comprehensive README with examples
- [x] JSON logging support (via `RUST_LOG_JSON=1`)
- [ ] Request/response payload logging (opt-in, sanitized) - Future

### HTTP Client Enhancements
- [ ] HTTP compression support (gzip, deflate)
  - [ ] Automatic Accept-Encoding header
  - [ ] Transparent decompression
- [ ] Retry logic with configurable strategies
  - [ ] Exponential backoff
  - [ ] Maximum retry attempts
  - [ ] Retry on specific errors (network, timeout)
- [ ] Cookie/Session handling
  - [ ] Cookie jar support
  - [ ] Session persistence across requests
- [ ] Custom HTTP headers API
  - [ ] Builder method for adding headers
  - [ ] Per-request header override
  - [ ] Example with API key authentication
- [ ] Connection pooling configuration
- [ ] Keep-alive settings

### WS-Security
- [ ] UsernameToken support
  - [ ] Plain text username/password
  - [ ] Password digest
  - [ ] Nonce and Created timestamp
- [ ] Timestamp validation
- [ ] Binary Security Token (X.509 certificates)
- [ ] Signature (digital signatures)
- [ ] Encryption (opt-in)
- [ ] SAML tokens (if needed)

**Example API:**
```rust
let client = Calculator::new("...")
    .with_username_token("user", "pass")
    .with_timestamp(Duration::from_secs(300));
```

### MTOM/XOP (Binary Attachments)
- [ ] Detect MTOM-enabled operations
- [ ] Base64 optimization
- [ ] Multipart HTTP handling
- [ ] Streaming for large files
- [ ] Example with file upload/download

### WSDL Import/Include
- [ ] Parse `<import>` elements
- [ ] Parse `<include>` elements
- [ ] Resolve relative paths
- [ ] Handle circular dependencies
- [ ] Merge schemas from multiple files

### Multiple Services
- [ ] Generate code for all services in WSDL
- [ ] Separate modules per service
- [ ] Shared type definitions
- [ ] Example with multi-service WSDL

---

## 🎯 Priority 4: Developer Experience (Ongoing)

### CLI Tool ✅
- [x] Standalone CLI for code generation (`soapus-cli`)
  - [x] `parse` command - Validate WSDL files
  - [x] `generate` command - Generate Rust code from WSDL
  - [x] `info` command - Display WSDL information
  - [x] Verbose output option
  - [x] Configurable output directory
  - [x] Custom client name option
  - [x] SOAP version selection (1.1/1.2)
- [ ] Watch mode for development
- [ ] Configuration file support (`.soap-codegen.toml`)

### Better Generated Code
- [ ] Optional: Use `chrono` for DateTime types
- [ ] Optional: Use `rust_decimal` for Decimal types
- [ ] Optional: Use `url` for URL types
- [ ] Custom type mappings via config
- [ ] Generate builder patterns for complex types
- [x] Generate documentation from WSDL `<documentation>` elements
  - [x] Extract `<wsdl:documentation>` from operations
  - [x] Include as Rust doc comments in generated code
  - [x] Maintain proper formatting and structure
  - [ ] Extract documentation from XSD types (future enhancement)
  - [ ] Extract field-level documentation (future enhancement)

### IDE Support
- [ ] rust-analyzer integration
- [ ] Better auto-completion hints
- [ ] Inline WSDL documentation in hover tooltips

### Debugging Tools
- [ ] `call_raw()` to get request/response XML
- [ ] Logging with `tracing` crate
- [ ] Request/response interceptors
- [ ] Mock server generation from WSDL

---

## 🎯 Priority 5: Optional Enhancements (Future)

### RPC/Encoded Style
- [ ] Support RPC/encoded SOAP (if needed)
- [ ] Most modern services use document/literal

### WS-Addressing
- [ ] Action, MessageID headers
- [ ] ReplyTo, FaultTo
- [ ] RelatesTo for correlation

### Advanced XSD Features
- [ ] `<choice>` compositor → Rust enum
- [ ] `<all>` compositor
- [ ] Type extension/restriction (inheritance)
- [ ] Abstract types
- [ ] Substitution groups
- [ ] Mixed content

### Custom Serialization
- [ ] Pluggable serializers
- [ ] Custom XML namespace handling
- [ ] Alternative to serde (yaserde?)

### Async Traits & Extensibility
- [ ] Make client operations overridable
- [ ] Custom middleware/interceptors
- [ ] Request/response transformation
- [ ] Hook points for custom logic
  - [ ] Pre-request hooks
  - [ ] Post-response hooks
  - [ ] Error handling hooks

---

## 📝 Documentation Tasks

### Examples
- [ ] Add more examples to `examples/`
  - [ ] WS-Security example
  - [ ] MTOM example
  - [ ] Multi-service example
  - [ ] Custom HTTP headers example
  - [ ] Error handling patterns example (timeouts, faults, network errors)
  - [ ] Retry and backoff example
  - [ ] Tracing/logging setup example
- [ ] Document each example thoroughly

### Guides
- [ ] Migration guide from other Rust SOAP libraries
- [ ] Performance tuning guide
- [ ] Troubleshooting guide
- [ ] Best practices guide

### Blog Posts / Articles
- [ ] Announcement post
- [ ] Technical deep-dive on WSDL parsing
- [ ] Comparison with other languages (.NET, Java)
- [ ] Use cases and success stories

---

## 🐛 Known Issues

### Current
- None! 🎉 All tests passing, example working

### Future Considerations
- XSD `choice` and `all` not implemented (rarely used)
- SimpleType restrictions partially implemented (enumerations work)
- No support for WSDL 2.0 (only 1.1)
- No support for RPC/encoded style (only document/literal)

---

## 📊 Metrics & Goals

### Test Coverage
- Current: ~80% (unit tests + integration tests)
- Goal: >90% for v1.0

### Compatibility
- Current: Works with Calculator WSDL
- Goal: Works with 20+ different real-world WSDLs

### Performance
- Current: Not benchmarked
- Goal: <1s for typical WSDL, <100ms for runtime calls

### Documentation
- Current: All major docs complete
- Goal: API docs on docs.rs, tutorials, videos

---

## 🎯 Priority 6: Code Quality & Tooling (Low Priority)

### Generated Code Quality
- [ ] Run rustfmt on generated code automatically
- [ ] Compile generated code in tests to catch errors early
- [ ] Add clippy lints to generated code
- [ ] Generate comprehensive doc comments from WSDL documentation

### Build-time Validation
- [ ] Validate generated code compiles before writing to disk
- [ ] Better error recovery during generation
- [ ] Incremental generation (only regenerate on WSDL changes)

---

## 🤝 Contribution Opportunities

Great places for contributors to help:

1. **Testing**: Try with your WSDL files and report issues
2. **WS-Security**: Implement UsernameToken support
3. **Examples**: Create examples for different services
4. **Documentation**: Improve guides and add tutorials
5. **CLI Tool Enhancements**: Add watch mode, config file support
6. **MTOM**: Implement binary attachment support
7. **Performance**: Profile and optimize
8. **HTTP Features**: Compression, retry logic, session handling

---

## 🔄 Update Schedule

This TODO is updated:
- After each major feature completion
- Weekly during active development
- After user feedback/issues

**Last Updated**: 2024-12-17
**Status**: 🎉 **Phase 1 + Phase 2 Complete!** (Core + Observability + CLI)
**Next Focus**: Production readiness, CI/CD, Publishing to crates.io

---

## 📝 Documentation Maintenance

**Important**: When making significant changes, update `.github/copilot-chat-context.md` to maintain context for future AI assistant chat sessions.

This includes:
- Architecture changes (like the parser refactorings)
- New features or capabilities
- Changed conventions or patterns
- Important decisions and their rationale
- New constraints or known issues

Additionally, update `.github/copilot-instructions.md` if coding conventions or patterns change.

These files help AI assistants (GitHub Copilot, Claude, ChatGPT) quickly understand the project state across different sessions.

---

**The SOAP client generator is fully functional and ready to use! 🚀**

Try it with `examples/calculator` or use it in your own projects!
