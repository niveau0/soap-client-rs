use std::fs;
use tempfile::tempdir;

#[test]
fn generates_code_from_wsdl() {
    let dir = tempdir().unwrap();
    let out_dir = dir.path().to_str().unwrap();

    soap_client_rs::generate_from_wsdl("examples/test.wsdl", out_dir).unwrap();

    let gen_file = dir.path().join("generated_soap_client.rs");
    let contents = fs::read_to_string(gen_file).unwrap();

    assert!(contents.contains("pub struct")); // einfacher Smoke-Test
}
