use clap::{Parser, Subcommand, ValueEnum};
use space_traders_sdk::{account::RegistrationRequest, faction::Factions};

use crate::{config::Agent, Application};

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
pub enum FactionArg {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Ethereal,
}

impl From<FactionArg> for Factions {
    fn from(arg: FactionArg) -> Self {
        match arg {
            FactionArg::Cosmic => Factions::Cosmic,
            FactionArg::Void => Factions::Void,
            FactionArg::Galactic => Factions::Galactic,
            FactionArg::Quantum => Factions::Quantum,
            FactionArg::Dominion => Factions::Dominion,
            FactionArg::Astro => Factions::Astro,
            FactionArg::Corsairs => Factions::Corsairs,
            FactionArg::Obsidian => Factions::Obsidian,
            FactionArg::Aegis => Factions::Aegis,
            FactionArg::United => Factions::United,
            FactionArg::Solitary => Factions::Solitary,
            FactionArg::Cobalt => Factions::Cobalt,
            FactionArg::Omega => Factions::Omega,
            FactionArg::Echo => Factions::Echo,
            FactionArg::Lords => Factions::Lords,
            FactionArg::Cult => Factions::Cult,
            FactionArg::Ancients => Factions::Ancients,
            FactionArg::Shadow => Factions::Shadow,
            FactionArg::Ethereal => Factions::Ethereal,
        }
    }
}

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

#[derive(Subcommand, Debug)]
pub enum AgentCommand {
    /// Show information for a given agent
    Info,
}

#[derive(Subcommand, Debug)]
pub enum AccountCommand {
    /// Register a new agent
    Register {
        /// Callsign of the agent
        #[arg(short, long)]
        callsign: String,

        /// Faction of the agent
        #[arg(short, long)]
        faction: FactionArg,
    },
    // List Agents registered to this account
    ListAgents,
}

#[derive(Subcommand, Debug)]
pub enum ContractCommand {
    /// List known contracts for a given agent
    List,
    /// Show info for a contract for a given agent
    Info {
        /// The ID of the contract
        #[arg(short, long)]
        id: String,
    },
    Accept {
        /// The ID of the contract
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Parser, Debug)]
#[command(name = "repl")]
pub struct ReplCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

async fn handle_register(
    application: &mut Application,
    callsign: String,
    faction: FactionArg,
) -> anyhow::Result<()> {
    let request: RegistrationRequest = RegistrationRequest {
        callsign: callsign.clone(),
        faction: Factions::from(faction),
    };

    match application.account.register_agent(request).await {
        Ok(agent) => {
            application.config.agents.push(Agent {
                id: agent.data.symbol.clone(),
                token: agent.get_token().unwrap().to_string(),
            });
            application.agents.insert(agent.data.symbol.clone(), agent);
            application.config.save()?;
            println!("Successfully registered agent {}", callsign);
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(anyhow::anyhow!("{:?}", e))
        }
    }
}

pub async fn handle_account_command(
    cmd: AccountCommand,
    application: &mut Application,
) -> anyhow::Result<()> {
    match cmd {
        AccountCommand::Register { callsign, faction } => {
            handle_register(application, callsign, faction).await?
        }
        AccountCommand::ListAgents => application
            .agents
            .iter()
            .for_each(|(callsign, _)| println!("{}", callsign)),
    }

    Ok(())
}

pub async fn handle_agent_command(
    callsign: String,
    cmd: AgentCommand,
    application: &mut Application,
) -> anyhow::Result<()> {
    match cmd {
        AgentCommand::Info => match application.agents.get(&callsign) {
            Some(agent) => {
                println!("{:?}", agent.data);
            }
            None => {
                println!("No known agent with that callsign");
            }
        },
    }

    Ok(())
}

pub async fn handle_contract_command(
    callsign: String,
    cmd: ContractCommand,
    application: &mut Application,
) -> anyhow::Result<()> {
    match cmd {
        ContractCommand::List => match application.agents.get(&callsign) {
            Some(agent) => {
                agent.list_contracts().for_each(|f| println!("{}", f));
            }
            None => {
                println!("No known agent with that callsign");
            }
        },
        ContractCommand::Info { id } => match application.agents.get_mut(&callsign) {
            Some(agent) => {
                println!("{:?}", agent.edit_contract(&id).data);
            }
            None => {
                println!("No known agent with that callsign");
            }
        },
        ContractCommand::Accept { id } => match application.agents.get_mut(&callsign) {
            Some(agent) => {
                let contract = agent.edit_contract(&id);
                match contract.accept().await {
                    Ok(_) => {
                        println!("Contract accepted: {:?}", contract.data);
                    }
                    Err(e) => {
                        println!("Failed to accept contract: {:?}", e);
                    }
                }
            }
            None => {
                println!("No known agent with that callsign");
            }
        },
    }

    Ok(())
}

pub async fn handle_command(cmd: Commands, application: &mut Application) -> anyhow::Result<()> {
    match cmd {
        Commands::Account { command } => handle_account_command(command, application).await,
        Commands::Agent { callsign, command } => {
            handle_agent_command(callsign, command, application).await
        }
        Commands::Contract { callsign, command } => {
            handle_contract_command(callsign, command, application).await
        }
    }
}
