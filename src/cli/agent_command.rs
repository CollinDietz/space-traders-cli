use clap::Subcommand;
use space_traders_sdk::agent::AgentData;

use crate::Application;

#[derive(Subcommand, Debug)]
pub enum AgentCommand {
    // List known Agents
    ListAgents,
    /// Show information for a given agent
    Info {
        /// Callsign of the agent
        #[arg(short, long)]
        callsign: String,
    },
}

pub fn print_agent_data(agent_data: &AgentData) {
    println!("");
    println!("Agent: {}", agent_data.symbol);
    println!("------------------------------");
    println!("  Headquarters: {}", agent_data.headquarters);
    println!("  Credits: {}", agent_data.credits);
    println!(
        "  Starting Faction: {}",
        serde_json::to_string(&agent_data.starting_faction).unwrap()
    );
    if let Some(ref account_id) = agent_data.account_id {
        println!("  Account ID: {}", account_id);
    }
}

impl AgentCommand {
    pub async fn handle(&self, application: &mut Application) -> anyhow::Result<()> {
        match self {
            AgentCommand::ListAgents => application
                .agents
                .iter()
                .for_each(|(callsign, _)| println!("{}", callsign)),
            AgentCommand::Info { callsign } => match application.agents.get(callsign) {
                Some(agent) => print_agent_data(&agent.data),
                None => {
                    println!("No known agent with that callsign");
                }
            },
        }

        Ok(())
    }
}
