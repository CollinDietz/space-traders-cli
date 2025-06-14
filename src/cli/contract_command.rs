use clap::Subcommand;

use crate::Application;

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

impl ContractCommand {
    pub async fn handle(
        &self,
        application: &mut Application,
        callsign: String,
    ) -> anyhow::Result<()> {
        match self {
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
}
