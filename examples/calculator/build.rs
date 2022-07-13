use soapus_codegen::SoapClientGenerator;
use std::path::PathBuf;

fn main() {
    // Get the directory containing Cargo.toml (examples/calculator/)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    // Navigate from examples/calculator/ to testdata/wsdl/calculator.wsdl
    let wsdl_path = PathBuf::from(&manifest_dir).join("../../testdata/wsdl/calculator.wsdl");

    println!("cargo:rerun-if-changed={}", wsdl_path.display());

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    SoapClientGenerator::builder()
        .wsdl_path(wsdl_path.to_str().expect("Invalid WSDL path"))
        .out_dir(&out_dir)
        .generate()
        .expect("Failed to generate SOAP client from WSDL");

    println!(
        "cargo:warning=SOAP client generated successfully in {}",
        out_dir
    );
}
