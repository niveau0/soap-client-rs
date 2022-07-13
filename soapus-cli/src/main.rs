//! SOAP Client CLI - Quick validation and code generation tool
//!
//! This CLI allows you to quickly test if a WSDL file can be parsed
//! and generate Rust code without needing to set up a build.rs.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use soapus_codegen::parser::parse_wsdl;
use soapus_codegen::SoapClientGenerator;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "soapus-cli")]
#[command(about = "soapus - WSDL Parser and Code Generator CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a WSDL file and validate its structure
    Parse {
        /// Path to the WSDL file
        #[arg(value_name = "WSDL_FILE")]
        wsdl_path: PathBuf,

        /// Show detailed parsing information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Generate Rust code from a WSDL file
    Generate {
        /// Path to the WSDL file
        #[arg(value_name = "WSDL_FILE")]
        wsdl_path: PathBuf,

        /// Output directory for generated code
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        output: PathBuf,

        /// Name of the generated client struct
        #[arg(short, long, value_name = "NAME")]
        client_name: Option<String>,

        /// SOAP version (1.1 or 1.2)
        #[arg(short, long, value_name = "VERSION")]
        soap_version: Option<String>,
    },

    /// Show information about a WSDL file
    Info {
        /// Path to the WSDL file
        #[arg(value_name = "WSDL_FILE")]
        wsdl_path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { wsdl_path, verbose } => {
            parse_command(wsdl_path, verbose)?;
        }
        Commands::Generate {
            wsdl_path,
            output,
            client_name,
            soap_version,
        } => {
            generate_command(wsdl_path, output, client_name, soap_version)?;
        }
        Commands::Info { wsdl_path } => {
            info_command(wsdl_path)?;
        }
    }

    Ok(())
}

fn parse_command(wsdl_path: PathBuf, verbose: bool) -> Result<()> {
    println!("🔍 Parsing WSDL file: {}", wsdl_path.display());

    let wsdl_content = fs::read_to_string(&wsdl_path)
        .with_context(|| format!("Failed to read WSDL file: {}", wsdl_path.display()))?;

    let model =
        parse_wsdl(&wsdl_content).map_err(|e| anyhow::anyhow!("Failed to parse WSDL: {}", e))?;

    println!("✅ WSDL parsed successfully!");

    if verbose {
        println!("\n📋 WSDL Details:");
        println!(
            "  Target Namespace: {}",
            model.target_namespace().unwrap_or("<none>")
        );
        println!("  Messages: {}", model.messages().len());
        println!("  Port Types: {}", model.port_types().len());
        println!("  Bindings: {}", model.bindings().len());
        println!("  Services: {}", model.services().len());

        if let Some(schema) = model.schema() {
            println!("\n📐 XSD Schema:");
            println!(
                "  Target Namespace: {}",
                schema.target_namespace.as_deref().unwrap_or("<none>")
            );
            println!("  Elements: {}", schema.elements.len());
            println!("  Complex Types: {}", schema.complex_types.len());
            println!("  Simple Types: {}", schema.simple_types.len());
        }
    }

    Ok(())
}

fn generate_command(
    wsdl_path: PathBuf,
    output: PathBuf,
    client_name: Option<String>,
    soap_version: Option<String>,
) -> Result<()> {
    println!("🔨 Generating code from WSDL: {}", wsdl_path.display());
    println!("📂 Output directory: {}", output.display());

    // Create output directory if it doesn't exist
    fs::create_dir_all(&output)
        .with_context(|| format!("Failed to create output directory: {}", output.display()))?;

    // Build generator
    let mut builder = SoapClientGenerator::builder()
        .wsdl_path(wsdl_path.to_str().context("Invalid WSDL path")?)
        .out_dir(output.to_str().context("Invalid output path")?);

    if let Some(name) = client_name {
        builder = builder.client_name(&name);
        println!("🏷️  Client name: {}", name);
    }

    if let Some(version) = soap_version {
        let soap_ver = match version.as_str() {
            "1.1" | "11" => soapus_codegen::SoapVersion::Soap11,
            "1.2" | "12" => soapus_codegen::SoapVersion::Soap12,
            _ => anyhow::bail!("Invalid SOAP version: {}. Use '1.1' or '1.2'", version),
        };
        builder = builder.soap_version(soap_ver);
        println!("📌 SOAP version: {}", version);
    }

    // Generate code
    builder.generate().context("Failed to generate code")?;

    let output_file = output.join("soap_client.rs");
    println!("✅ Code generated successfully!");
    println!("📄 Output file: {}", output_file.display());

    Ok(())
}

fn info_command(wsdl_path: PathBuf) -> Result<()> {
    println!("ℹ️  WSDL Information: {}", wsdl_path.display());
    println!();

    let wsdl_content = fs::read_to_string(&wsdl_path)
        .with_context(|| format!("Failed to read WSDL file: {}", wsdl_path.display()))?;

    let model =
        parse_wsdl(&wsdl_content).map_err(|e| anyhow::anyhow!("Failed to parse WSDL: {}", e))?;

    // Service information
    println!("🌐 Services:");
    for service in model.services() {
        for port in &service.ports {
            println!("  • {} ({})", service.name, port.address);
        }
    }

    // Operations information
    println!("\n📡 Operations:");
    for port_type in model.port_types() {
        println!("  Port Type: {}", port_type.name);
        for operation in &port_type.operations {
            let input = operation
                .input
                .as_ref()
                .map(|q| q.local_name())
                .unwrap_or("<none>");
            let output = operation
                .output
                .as_ref()
                .map(|q| q.local_name())
                .unwrap_or("<none>");

            println!("    • {} ({} → {})", operation.name, input, output);
        }
    }

    // Type information
    if let Some(schema) = model.schema() {
        println!("\n📐 Types:");
        if !schema.complex_types.is_empty() {
            println!("  Complex Types: {}", schema.complex_types.len());
            for (name, _) in schema.complex_types.iter().take(5) {
                println!("    • {}", name);
            }
            if schema.complex_types.len() > 5 {
                println!("    ... and {} more", schema.complex_types.len() - 5);
            }
        }

        if !schema.simple_types.is_empty() {
            println!("  Simple Types: {}", schema.simple_types.len());
            for (name, _) in schema.simple_types.iter().take(5) {
                println!("    • {}", name);
            }
            if schema.simple_types.len() > 5 {
                println!("    ... and {} more", schema.simple_types.len() - 5);
            }
        }
    }

    println!("\n✅ Analysis complete!");

    Ok(())
}
