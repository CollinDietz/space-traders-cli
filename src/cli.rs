use clap::{Parser, Subcommand, ValueEnum};
use space_traders_sdk::{
    faction::Factions,
    sdk::{login::RegistrationRequest, Sdk},
};

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
}

#[derive(Parser, Debug)]
#[command(name = "repl")]
pub struct ReplCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

async fn handle_register(sdk: &Sdk, callsign: String, faction: FactionArg) {
    let request: RegistrationRequest = RegistrationRequest {
        callsign: callsign.clone(),
        faction: Factions::from(faction),
    };

    match sdk.register(request).await {
        Ok(login_data) => {
            println!("{:?}", login_data);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

pub async fn handle_command(cmd: Commands, sdk: &Sdk) -> anyhow::Result<()> {
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
            handle_register(sdk, callsign, faction).await;
        }
    }
    Ok(())
}
