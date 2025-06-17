use clap::{Parser, Subcommand};

use crate::{
    cli::{
        account_command::AccountCommand, agent_command::AgentCommand,
        contract_command::ContractCommand, system_command::SystemCommand,
    },
    Application,
};

mod account_command;
mod agent_command;
mod contract_command;
mod system_command;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Account level commands
    Account {
        #[command(subcommand)]
        command: AccountCommand,
    },
    /// Agent level commands
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
    /// Contract level commands
    Contract {
        #[command(subcommand)]
        command: ContractCommand,
    },
    /// System level commands
    System {
        #[command(subcommand)]
        command: SystemCommand,
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
        Commands::Agent { command } => command.handle(application).await,
        Commands::Contract { command } => command.handle(application).await,
        Commands::System { command } => command.handle(application).await,
    }
}
