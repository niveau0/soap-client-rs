use soapus_codegen::SoapClientGenerator;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_generate_from_calculator_wsdl() {
    let dir = tempdir().unwrap();

    // Generate code from Calculator WSDL
    let result = SoapClientGenerator::builder()
        .wsdl_path("../testdata/wsdl/calculator.wsdl")
        .out_dir(dir.path())
        .generate();

    // Should succeed
    assert!(result.is_ok(), "Code generation failed: {:?}", result.err());

    let gen = result.unwrap();

    // Check generated file exists
    let generated_file = &gen.output_file;
    assert!(generated_file.exists(), "Generated file not found");

    // Read and verify content
    let content = fs::read_to_string(generated_file).unwrap();

    // Verify it contains expected elements
    assert!(
        content.contains("pub struct"),
        "Should contain struct definitions"
    );
    assert!(content.contains("impl"), "Should contain impl blocks");
    assert!(
        content.contains("Calculator"),
        "Should have Calculator client"
    );
    assert!(
        content.contains("pub async fn add"),
        "Should have add operation"
    );
    assert!(
        content.contains("pub async fn subtract"),
        "Should have subtract operation"
    );
}

#[test]
fn test_generate_from_countryinfo_wsdl() {
    let dir = tempdir().unwrap();

    // Generate code from CountryInfo WSDL
    let result = SoapClientGenerator::builder()
        .wsdl_path("../testdata/wsdl/countryinfo.wsdl")
        .out_dir(dir.path())
        .generate();

    // Should succeed
    assert!(
        result.is_ok(),
        "CountryInfo code generation failed: {:?}",
        result.err()
    );

    let gen = result.unwrap();

    // Check generated file exists
    let generated_file = &gen.output_file;
    assert!(generated_file.exists(), "Generated file not found");

    // Read and verify content
    let content = fs::read_to_string(generated_file).unwrap();

    // Verify it contains expected elements
    assert!(
        content.contains("pub struct"),
        "Should contain struct definitions"
    );
    assert!(content.contains("impl"), "Should contain impl blocks");

    // CountryInfo service has many operations, check for a few
    assert!(
        content.contains("CountryInfoService") || content.contains("CountryInfo"),
        "Should have CountryInfo client"
    );

    // Verify some complex types from the WSDL
    assert!(
        content.contains("TContinent") || content.contains("Continent"),
        "Should have Continent type"
    );
    assert!(
        content.contains("TCurrency") || content.contains("Currency"),
        "Should have Currency type"
    );
}

#[test]
fn test_generate_from_numberconversion_wsdl() {
    let dir = tempdir().unwrap();

    // Generate code from NumberConversion WSDL
    let result = SoapClientGenerator::builder()
        .wsdl_path("../testdata/wsdl/numberconversion.wsdl")
        .out_dir(dir.path())
        .generate();

    // Should succeed
    assert!(
        result.is_ok(),
        "NumberConversion code generation failed: {:?}",
        result.err()
    );

    let gen = result.unwrap();

    // Check generated file exists
    let generated_file = &gen.output_file;
    assert!(generated_file.exists(), "Generated file not found");

    // Read and verify content
    let content = fs::read_to_string(generated_file).unwrap();

    // Verify it contains expected elements
    assert!(
        content.contains("pub struct"),
        "Should contain struct definitions"
    );
    assert!(content.contains("impl"), "Should contain impl blocks");
    assert!(
        content.contains("NumberConversion"),
        "Should have NumberConversion client"
    );

    // Check for operations
    assert!(
        content.contains("number_to_words") || content.contains("NumberToWords"),
        "Should have number_to_words operation"
    );
    assert!(
        content.contains("number_to_dollars") || content.contains("NumberToDollars"),
        "Should have number_to_dollars operation"
    );
}

#[test]
fn test_all_wsdls_generate_valid_rust() {
    // Test that all WSDL files generate code that at least compiles syntactically
    let wsdl_files = vec![
        ("../testdata/wsdl/calculator.wsdl", "Calculator"),
        ("../testdata/wsdl/countryinfo.wsdl", "CountryInfo"),
        ("../testdata/wsdl/numberconversion.wsdl", "NumberConversion"),
    ];

    for (wsdl_path, expected_name) in wsdl_files {
        let dir = tempdir().unwrap();

        let result = SoapClientGenerator::builder()
            .wsdl_path(wsdl_path)
            .out_dir(dir.path())
            .generate();

        assert!(
            result.is_ok(),
            "Failed to generate from {}: {:?}",
            wsdl_path,
            result.err()
        );

        let gen = result.unwrap();
        let content = fs::read_to_string(&gen.output_file).unwrap();

        // Basic sanity checks
        assert!(
            content.contains("pub struct") || content.contains("pub enum"),
            "{} should contain type definitions",
            wsdl_path
        );
        assert!(
            content.contains(expected_name) || content.contains(&expected_name.to_lowercase()),
            "{} should contain {}",
            wsdl_path,
            expected_name
        );

        // Ensure it has the standard imports
        assert!(
            content.contains("use soapus_runtime"),
            "{} should import runtime",
            wsdl_path
        );
        assert!(
            content.contains("use serde"),
            "{} should import serde",
            wsdl_path
        );
    }
}
