use clap::{Parser, Subcommand};

use crate::{
    cli::{
        account_command::AccountCommand, agent_command::AgentCommand,
        contract_command::ContractCommand,
    },
    Application,
};

mod account_command;
mod agent_command;
mod contract_command;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Account level commands
    Account {
        #[command(subcommand)]
        command: AccountCommand,
    },
    /// Agent level commands
    Agent {
        #[arg(short, long)]
        callsign: String,
        #[command(subcommand)]
        command: AgentCommand,
    },
    Contract {
        #[arg(short, long)]
        callsign: String,
        #[command(subcommand)]
        command: ContractCommand,
    },
}

#[derive(Parser, Debug)]
#[command(name = "repl")]
pub struct ReplCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub async fn handle_command(cmd: Commands, application: &mut Application) -> anyhow::Result<()> {
    match cmd {
        Commands::Account { command } => command.handle(application).await,
        Commands::Agent { callsign, command } => command.handle(application, callsign).await,
        Commands::Contract { callsign, command } => command.handle(application, callsign).await,
    }
}
