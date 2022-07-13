# soapus-cli

[![Crates.io](https://img.shields.io/crates/v/soapus-cli.svg)](https://crates.io/crates/soapus-cli)
[![Documentation](https://docs.rs/soapus-cli/badge.svg)](https://docs.rs/soapus-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Command-line tool for parsing WSDL files and generating Rust SOAP client code.

**Part of the [soapus](https://github.com/niveau0/soapus) project.**

## 🚀 Installation

### From crates.io

```bash
cargo install soapus-cli
```

### From source

```bash
git clone https://github.com/niveau0/soapus.git
cd soapus
cargo build --release -p soapus-cli
```

The binary will be available at `target/release/soapus-cli`.

## Commands

```bash
# Parse and validate WSDL
soapus-cli parse service.wsdl [--verbose]

# Generate Rust code
soapus-cli generate service.wsdl [--output DIR] [--client-name NAME] [--soap-version 1.1|1.2]

# Show WSDL information
soapus-cli info service.wsdl
```

## Use Cases

- **Quick validation**: Test if a WSDL can be parsed before using it
- **Exploration**: Inspect operations and types in an unknown WSDL
- **Code inspection**: Generate code to see what will be created
- **Testing**: Try different configurations (SOAP versions, client names)

## Documentation

📚 **Full documentation in the [main repository](https://github.com/niveau0/soapus)**:

- [Quick Start Guide](https://github.com/niveau0/soapus/blob/main/QUICKSTART.md)
- [Working Examples](https://github.com/niveau0/soapus/tree/main/examples) - See `build.rs` integration

## License

Licensed under the [MIT License](https://opensource.org/licenses/MIT).