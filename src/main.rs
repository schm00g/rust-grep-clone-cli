#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "src/main.rs";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
