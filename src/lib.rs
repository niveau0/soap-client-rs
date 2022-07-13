// src/lib.rs
pub mod codegen;
pub mod parser;

use std::fs;

pub fn generate_from_wsdl(
    wsdl_path: &str,
    out_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let wsdl_xml = fs::read_to_string(wsdl_path)?;

    let wsdl_def = parser::wsdl::parse_wsdl(&wsdl_xml)?;
    dbg!(&wsdl_def);
    // let generated_code = codegen::generator::generate_code(&wsdl_def.into_codegen_model());
    // 
    // let target_file = format!("{}/generated_soap_client.rs", out_dir);
    // fs::write(&target_file, generated_code)?;

    Ok(())
}
