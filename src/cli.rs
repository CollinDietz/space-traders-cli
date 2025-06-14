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
        #[command(subcommand)]
        command: AgentCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum AgentCommand {
    /// List agents
    ListAgents,
    /// Get info about a specific agent
    GetMyAgent {
        #[arg(short, long)]
        callsign: String,
    },
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
    }

    Ok(())
}

pub async fn handle_agent_command(
    cmd: AgentCommand,
    application: &mut Application,
) -> anyhow::Result<()> {
    match cmd {
        AgentCommand::ListAgents => application
            .agents
            .iter()
            .for_each(|(callsign, _)| println!("{}", callsign)),
        AgentCommand::GetMyAgent { callsign } => {
            match application
                .agents
                .iter()
                .find(|(agent_callsign, _)| **agent_callsign == callsign)
            {
                Some((_, agent)) => {
                    println!("{:?}", agent.data);
                }
                None => {
                    println!("No known agent with that callsign");
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_command(cmd: Commands, application: &mut Application) -> anyhow::Result<()> {
    match cmd {
        Commands::Account { command } => handle_account_command(command, application).await,
        Commands::Agent { command } => handle_agent_command(command, application).await,
    }
}
