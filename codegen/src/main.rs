//! Swagger 2.0 code generator for LXD types
//!
//! This tool parses the LXD REST API Swagger 2.0 specification and generates
//! Rust type definitions for use in the lxd-types crate.

mod generator;
mod parser;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lxd-codegen")]
#[command(about = "Generate Rust types from LXD Swagger 2.0 specification")]
struct Args {
    /// Path to the Swagger 2.0 YAML specification file
    #[arg(short, long, default_value = "codegen/openapi/rest-api.yaml")]
    input: PathBuf,

    /// Output directory for generated Rust files
    #[arg(short, long, default_value = "crates/lxd-types/src/generated")]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("Parsing Swagger 2.0 spec: {:?}", args.input);
    let spec = parser::parse_openapi(&args.input)?;

    println!("Found {} definitions", spec.definitions.len());
    println!("Found {} paths", spec.paths.len());

    println!("Generating Rust types to: {:?}", args.output);
    generator::generate(&spec, &args.output)?;

    println!("Done!");
    Ok(())
}
