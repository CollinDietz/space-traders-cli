use clap::Parser;

mod cli;
mod repl;
mod utils;

#[derive(Parser)]
#[command(name = "Space Traders CLI")]
#[command(about = "A CLI tool to interact with Space Traders SDK", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => cli::handle_command(cmd).await?,
        None => repl::start().await?,
    }
    Ok(())
}
