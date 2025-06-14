use clap::Subcommand;

use crate::Application;

#[derive(Subcommand, Debug)]
pub enum AgentCommand {
    /// Show information for a given agent
    Info,
}

impl AgentCommand {
    pub async fn handle(
        &self,
        application: &mut Application,
        callsign: String,
    ) -> anyhow::Result<()> {
        match self {
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
}
