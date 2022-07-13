# soapus-codegen

[![Crates.io](https://img.shields.io/crates/v/soapus-codegen.svg)](https://crates.io/crates/soapus-codegen)
[![Documentation](https://docs.rs/soapus-codegen/badge.svg)](https://docs.rs/soapus-codegen)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Code generator for SOAP clients from WSDL files. Parses WSDL 1.1 and embedded XSD schemas to generate type-safe Rust code.

**Part of the [soapus](https://github.com/niveau0/soapus) project.**

## Installation

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
soapus-codegen = "0.1"
```

## Quick Example

In your `build.rs`:

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

In your code:

```rust
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));
```

## Documentation

📚 **Full documentation and examples in the [main repository](https://github.com/niveau0/soapus)**:

- [Quick Start Guide](https://github.com/niveau0/soapus/blob/main/QUICKSTART.md) - Get started in 5 minutes
- [Working Examples](https://github.com/niveau0/soapus/tree/main/examples) - Calculator and observability examples
- [API Documentation](https://docs.rs/soapus-codegen) - Complete API reference

## Features

- ✅ WSDL 1.1 parsing
- ✅ XSD schema support (ComplexType, SimpleType, sequences, enumerations)
- ✅ Idiomatic Rust code generation
- ✅ Build-time integration
- ✅ Automatic type mapping (XSD → Rust)
- ✅ Serde serialization support

## Runtime Dependency

Generated code requires:

```toml
[dependencies]
soapus-runtime = "0.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

## License

Licensed under the [MIT License](https://opensource.org/licenses/MIT).