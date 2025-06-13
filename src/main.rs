use clap::Parser;
use config::Config;
use space_traders_sdk::{account::Account, agent::Agent};

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

pub struct Application {
    pub config: Config,
    pub account: Account,
    pub agents: Vec<Agent>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::load()?;

    if config.account_token.is_empty() {
        println!("No token found. Please enter your API token:");
        let token = prompt_password("Token: ")?;
        config.account_token = token.trim().to_string();
        config.save()?;
    }

    let mut application = Application {
        account: Account::new(config.account_token.clone()),
        config: config,
        agents: vec![],
    };

    // TODO: make this possible
    // config.agents.iter().for_each(|f| {
    //     application.agents.push(Agent::ne);
    //     sdk.add_agent_token(f.id.clone(), f.token.clone());
    // });

    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => cli::handle_command(cmd, &mut application).await?,
        None => repl::start(&mut application).await?,
    }
    Ok(())
}
