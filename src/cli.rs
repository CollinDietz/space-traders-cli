use clap::{Parser, Subcommand, ValueEnum};
use space_traders_sdk::{
    faction::Factions,
    sdk::{login::RegistrationRequest, Sdk},
};

use crate::config::{Agent, Config};

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
    /// Example command
    GetSomething {
        #[arg(short, long)]
        id: String,
    },
    /// Example command
    DoSomething {
        #[arg(short, long)]
        force: bool,
    },
    /// Register a new agent
    Register {
        /// Callsign of the agent
        #[arg(short, long)]
        callsign: String,

        ///Faction of the agent
        #[arg(short, long)]
        faction: FactionArg,
    },
    /// List agents
    ListAgents,
    GetMyAgent {
        #[arg(short, long)]
        callsign: String,
    },
}

#[derive(Parser, Debug)]
#[command(name = "repl")]
pub struct ReplCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

async fn handle_register(
    sdk: &mut Sdk,
    config: &mut Config,
    callsign: String,
    faction: FactionArg,
) -> anyhow::Result<()> {
    let request: RegistrationRequest = RegistrationRequest {
        callsign: callsign.clone(),
        faction: Factions::from(faction),
    };

    match sdk.register(request).await {
        Ok(login_data) => {
            println!("{:?}", login_data);
            config.agents.push(Agent {
                id: login_data.agent.symbol.clone(),
                token: login_data.token,
            });
            config.save()?;
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn handle_command(
    cmd: Commands,
    sdk: &mut Sdk,
    config: &mut Config,
) -> anyhow::Result<()> {
    match cmd {
        Commands::GetSomething { id } => {
            let result = format!(r#"{{"status":"ok","id":"{}"}}"#, id);
            crate::utils::print_json_pretty(&result);
        }
        Commands::DoSomething { force } => {
            let result = serde_json::json!({ "done": true, "forced": force });
            crate::utils::print_json_value(&result);
        }
        Commands::Register { callsign, faction } => {
            handle_register(sdk, config, callsign, faction).await?;
        }
        Commands::ListAgents => {
            config.agents.iter().for_each(|f| println!("{}", f.id));
        }
        Commands::GetMyAgent { callsign } => match sdk.get_agent(callsign).await {
            Ok(agent) => {
                println!("{:?}", agent);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        },
    }
    Ok(())
}
