use clap::Parser;

mod cli;
mod repl;
mod utils;
mod sdk;

#[derive(Parser)]
#[command(name = "mytool")]
#[command(about = "An SDK CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(cmd) => cli::handle_command(cmd)?,
        None => repl::start()?,
    }
    Ok(())
}
