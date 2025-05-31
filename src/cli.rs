use clap::{Subcommand, Parser};

#[derive(Subcommand, Debug)]
pub enum Commands {
    GetSomething {
        #[arg(short, long)]
        id: String,
    },
    DoSomething {
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Parser, Debug)]
#[command(name = "repl")]
pub struct ReplCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub async fn handle_command(cmd: Commands) -> anyhow::Result<()> {
    match cmd {
        Commands::GetSomething { id } => {
            let result = format!(r#"{{"status":"ok","id":"{}"}}"#, id);
            crate::utils::print_json_pretty(&result);
        }
        Commands::DoSomething { force } => {
            let result = serde_json::json!({ "done": true, "forced": force });
            crate::utils::print_json_value(&result);
        }
    }
    Ok(())
}
