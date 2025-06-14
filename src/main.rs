use std::{collections::HashMap, sync::Arc};

use clap::Parser;
use config::Config;
use space_traders_sdk::{account::Account, agent::Agent, space_traders_client::SpaceTradersClient};

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
use std::io::{self, Write};

pub struct Application {
    pub config: Config,
    pub account: Account,
    pub agents: HashMap<String, Agent>,
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

    let client = Arc::new(SpaceTradersClient::new(Some(config.account_token.clone())));

    print!("Loading details for known agents...");
    io::stdout().flush().unwrap();
    let agent_futures = config.agents.iter().map(|agent| {
        let client = Arc::new(SpaceTradersClient::clone_with_token(&client, &agent.token));
        let id = agent.id.clone();
        async move { (id, Agent::new(client).await.unwrap()) }
    });

    let agents_vec = futures::future::join_all(agent_futures).await;
    let agents: HashMap<String, Agent> = agents_vec.into_iter().collect();
    println!("done");

    let mut application = Application {
        account: Account::new(client.clone()),
        config: config,
        agents,
    };

    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => cli::handle_command(cmd, &mut application).await?,
        None => repl::start(&mut application).await?,
    }
    Ok(())
}
