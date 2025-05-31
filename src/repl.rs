use clap::{CommandFactory, Parser};
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{error::ReadlineError, Editor};
use rustyline::{Context, Helper};
use space_traders_sdk::sdk::Sdk;

const HISTORY_PATH: &str = ".mytool_history";

pub struct ReplHelper {
    pub commands: Vec<String>,
}

impl Completer for ReplHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let completions = self
            .commands
            .iter()
            .filter(|cmd| cmd.starts_with(line))
            .map(|cmd| Pair {
                display: cmd.clone(),
                replacement: cmd.clone(),
            })
            .collect();
        Ok((0, completions))
    }
}
impl Hinter for ReplHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}
impl Highlighter for ReplHelper {}
impl Validator for ReplHelper {
    fn validate(&self, _ctx: &mut ValidationContext<'_>) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}
impl Helper for ReplHelper {}

pub async fn start(sdk: &Sdk) -> anyhow::Result<()> {
    let mut commands = crate::cli::ReplCli::command()
        .get_subcommands()
        .map(|sc| sc.get_name().to_string())
        .collect::<Vec<_>>();
    commands.extend(["exit".into(), "help".into()]);
    let helper = ReplHelper { commands };
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));
    if rl.load_history(HISTORY_PATH).is_err() {
        println!("(No previous history)");
    }

    println!("Type 'help' or 'exit'.");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if let Err(e) = handle_input(sdk, line).await {
                    eprintln!("Error: {e}");
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => return Err(err.into()),
        }
    }

    rl.save_history(HISTORY_PATH)?;
    Ok(())
}

async fn handle_input(sdk: &Sdk, line: String) -> anyhow::Result<()> {
    let args = shell_words::split(&line)?;
    if args.is_empty() {
        return Ok(());
    }

    if args[0] == "exit" {
        std::process::exit(0);
    } else if args[0] == "help" {
        println!("Available commands:");
        crate::cli::ReplCli::command().print_help()?;
        println!();
        return Ok(());
    }

    match crate::cli::ReplCli::try_parse_from(
        std::iter::once("repl").chain(args.iter().map(String::as_str)),
    ) {
        Ok(parsed) => {
            if let Some(cmd) = parsed.command {
                crate::cli::handle_command(cmd, sdk).await?;
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(())
}
