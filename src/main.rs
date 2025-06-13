use clap::Parser;
use config::Config;
use space_traders_sdk::sdk::Sdk;

mod cli;
mod config;
mod repl;
mod utils;

#[derive(Parser)]
#[command(name = "Space Traders CLI")]
#[command(about = "A CLI tool to interact with Space Traders SDK", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

use rpassword::prompt_password;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let token = get_token()?;
    let mut config = Config::load()?;

    if config.account_token.is_empty() {
        println!("No token found. Please enter your API token:");
        let token = prompt_password("Token: ")?;
        config.account_token = token.trim().to_string();
        config.save()?;
    }

    let mut sdk = Sdk::new(config.account_token.clone());

    config.agents.iter().for_each(|f| {
        sdk.add_agent_token(f.id.clone(), f.token.clone());
    });

    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => cli::handle_command(cmd, &mut sdk, &mut config).await?,
        None => repl::start(&mut sdk, &mut config).await?,
    }
    Ok(())
}
