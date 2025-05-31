use clap::Parser;
use space_traders_sdk::sdk::Sdk;

mod cli;
mod repl;
mod utils;

#[derive(Parser)]
#[command(name = "Space Traders CLI")]
#[command(about = "A CLI tool to interact with Space Traders SDK", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

use dirs::config_dir;
use rpassword::prompt_password;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

/// Get the token from ~/.config/mytool/token or prompt the user and save it
pub fn get_token() -> io::Result<String> {
    // Step 1: Find the config path
    let token_file = config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mytool/token");

    // Step 2: If token file exists, read and return it
    if token_file.exists() {
        let token = fs::read_to_string(&token_file)?.trim().to_string();
        return Ok(token);
    }

    // Step 3: Prompt for the token (input hidden)
    println!("No token found. Please enter your API token:");
    let token = prompt_password("Token: ")?;

    // Step 4: Save the token to the config directory
    if let Some(parent) = token_file.parent() {
        fs::create_dir_all(parent)?; // Make sure the directory exists
    }

    fs::write(&token_file, &token)?;

    println!("Token saved to: {}", token_file.display());
    Ok(token)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = get_token()?;
    let sdk = Sdk::new(token);

    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => cli::handle_command(cmd, &sdk).await?,
        None => repl::start(&sdk).await?,
    }
    Ok(())
}
