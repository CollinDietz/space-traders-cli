use clap::Subcommand;
use space_traders_sdk::contract::ContractData;

use crate::Application;

#[derive(Subcommand, Debug)]
pub enum ContractCommand {
    /// List known contracts for a given agent
    List {
        /// Callsign of the agent
        #[arg(short, long)]
        callsign: String,
    },
    /// Show info for a contract for a given agent
    Info {
        /// Callsign of the agent
        #[arg(short, long)]
        callsign: String,
        /// The ID of the contract
        #[arg(short, long)]
        id: String,
    },
    /// Accept a given contract
    Accept {
        /// Callsign of the agent
        #[arg(short, long)]
        callsign: String,
        /// The ID of the contract
        #[arg(short, long)]
        id: String,
    },
}

fn display_contract(contract: &ContractData) {
    println!("");
    println!("Contract ID: {}", contract.id);
    println!("------------------------------");
    println!(
        "Type: {}",
        serde_json::to_string(&contract.contract_type).unwrap()
    );
    println!(
        "Faction: {}",
        serde_json::to_string(&contract.faction).unwrap()
    );
    println!("Terms:");
    println!("  Deadline: {}", contract.terms.deadline);
    println!(
        "  Payment: {} up front, {} on completion",
        contract.terms.payment.on_accepted, contract.terms.payment.on_fulfilled
    );
    println!("  Deliverables:");
    if let Some(deliverables) = &contract.terms.deliver {
        for deliverable in deliverables {
            println!(
                "    {} of {} {} to {}",
                deliverable.units_fulfilled,
                deliverable.units_required,
                serde_json::to_string(&deliverable.trade_symbol).unwrap(),
                serde_json::to_string(&deliverable.destination_symbol).unwrap()
            );
        }
    } else {
        println!("    None");
    }
    println!("Accepted: {}", if contract.accepted { "yes" } else { "no" });
    println!(
        "Fulfilled: {}",
        if contract.fulfilled { "yes" } else { "no" }
    );
    println!();
}

fn display_contract_short(contract: &ContractData) {
    println!(
        "ID: {} | Type: {} | Faction: {} | Accepted: {} | Fulfilled: {}",
        contract.id,
        serde_json::to_string(&contract.contract_type).unwrap(),
        serde_json::to_string(&contract.faction).unwrap(),
        if contract.accepted { "yes" } else { "no" },
        if contract.fulfilled { "yes" } else { "no" }
    );
}

impl ContractCommand {
    pub async fn handle(&self, application: &mut Application) -> anyhow::Result<()> {
        match self {
            ContractCommand::List { callsign } => match application.agents.get(callsign) {
                Some(agent) => {
                    agent
                        .contracts()
                        .for_each(|f| display_contract_short(&f.1.data));
                }
                None => {
                    println!("No known agent with that callsign");
                }
            },
            ContractCommand::Info { callsign, id } => match application.agents.get_mut(callsign) {
                Some(agent) => display_contract(&agent.edit_contract(&id).data),
                None => {
                    println!("No known agent with that callsign");
                }
            },
            ContractCommand::Accept { callsign, id } => {
                match application.agents.get_mut(callsign) {
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
                }
            }
        }

        Ok(())
    }
}
