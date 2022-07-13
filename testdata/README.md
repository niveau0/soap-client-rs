# Test Data

This directory contains test resources used across the workspace for testing the SOAP client generator.

## Structure

```
testdata/
└── wsdl/           # WSDL files for testing
    ├── calculator.wsdl
    ├── countryinfo.wsdl
    └── numberconversion.wsdl
```

## WSDL Test Files

### calculator.wsdl
- **Source**: http://www.dneonline.com/calculator.asmx?wsdl
- **Description**: Simple calculator service with 4 operations (Add, Subtract, Multiply, Divide)
- **Used in**:
  - Unit tests (`soapus-codegen/src/parser/wsdl/parser.rs`)
  - Integration tests (`soapus-codegen/tests/integration_test.rs`)
  - Calculator example (`examples/calculator/`)
- **Complexity**: Basic
- **Features**: Document/literal style, simple types only

### countryinfo.wsdl
- **Source**: http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL
- **Description**: Country information service with multiple complex operations
- **Used in**:
  - Unit tests
  - Integration tests
- **Complexity**: Medium
- **Features**: Complex types, arrays, nested structures

### numberconversion.wsdl
- **Source**: https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL
- **Description**: Number to words conversion service
- **Used in**:
  - Unit tests
  - Integration tests
- **Complexity**: Medium
- **Features**: String operations, multiple operations

## Usage in Tests

### Unit Tests (include_str!)

```rust
#[test]
fn parses_calculator_wsdl() {
    let wsdl = include_str!("../../../../../testdata/wsdl/calculator.wsdl");
    let model = parse_wsdl(wsdl).unwrap();
    // ...
}
```

### Integration Tests (file path)

```rust
#[test]
fn test_generate_from_calculator_wsdl() {
    let result = SoapClientGenerator::builder()
        .wsdl_path("../testdata/wsdl/calculator.wsdl")
        .out_dir(dir.path())
        .generate();
    // ...
}
```

### Examples (copied to example directory)

The calculator example has its own copy of `calculator.wsdl` as `service.wsdl` to demonstrate standalone usage.

## Adding New Test WSDLs

When adding new WSDL files for testing:

1. Place them in `testdata/wsdl/`
2. Use descriptive names (service-name.wsdl)
3. Add documentation here with:
   - Source URL
   - Description
   - Complexity level
   - Notable features
4. Update tests to reference the new file

## Gitignore

All files in this directory are tracked in git as they are essential for testing.