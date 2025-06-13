use clap::{Parser, Subcommand, ValueEnum};
use space_traders_sdk::{account::RegistrationRequest, faction::Factions};

use crate::Application;

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
            // application.config.agents.push( Agent { id: agent.data.symbol, token: "".into() });
            application.agents.push(agent);
            // application.config.save()?;
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(anyhow::anyhow!("{:?}", e))
        }
    }
}

pub async fn handle_command(cmd: Commands, application: &mut Application) -> anyhow::Result<()> {
    match cmd {
        Commands::Register { callsign, faction } => {
            handle_register(application, callsign, faction).await?;
        }
        Commands::ListAgents => {
            application
                .agents
                .iter()
                .for_each(|f| println!("{}", f.data.symbol));
        }
        Commands::GetMyAgent { callsign } => match application
            .agents
            .iter()
            .find(|agent| agent.data.symbol == callsign)
        {
            Some(agent) => {
                println!("{:?}", agent);
            }
            None => {
                println!("No known agent with that callsign");
            }
        },
    }
    Ok(())
}
