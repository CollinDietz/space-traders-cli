use clap::{Subcommand, ValueEnum};
use space_traders_sdk::{account::RegistrationRequest, faction::Factions};

use crate::{config::Agent, Application};

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

impl AccountCommand {
    pub async fn handle(&self, application: &mut Application) -> anyhow::Result<()> {
        match self {
            AccountCommand::Register { callsign, faction } => {
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
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return Err(anyhow::anyhow!("{:?}", e));
                    }
                }
            }
        }

        Ok(())
    }
}

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

impl From<&FactionArg> for Factions {
    fn from(arg: &FactionArg) -> Self {
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
