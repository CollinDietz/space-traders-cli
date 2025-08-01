use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use directories::ProjectDirs;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{error::ReadlineError, Editor};
use rustyline::{Context, Helper};

use crate::Application;

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

fn history_path() -> PathBuf {
    let path = ProjectDirs::from("com", "CollinDietz", "space-traders-cli")
        .expect("No valid home directory found")
        .config_dir()
        .join("history.txt");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create config directory");
    }

    path
}

pub async fn start(application: &mut Application) -> anyhow::Result<()> {
    fn collect_command_names(cmd: &clap::Command, prefix: String, names: &mut Vec<String>) {
        // Collect flags/options for this command
        for opt in cmd.get_opts() {
            let flag = if let Some(short) = opt.get_short() {
                format!("{} -{}", prefix, short)
            } else if let Some(long) = opt.get_long() {
                format!("{} --{}", prefix, long)
            } else {
                continue;
            };
            names.push(flag.trim().to_string());
        }
        // Collect subcommands recursively
        for sub in cmd.get_subcommands() {
            let name = if prefix.is_empty() {
                sub.get_name().to_string()
            } else {
                format!("{} {}", prefix, sub.get_name())
            };
            names.push(name.clone());
            collect_command_names(sub, name, names);
        }
    }

    let mut commands = Vec::new();
    let root_cmd = crate::cli::ReplCli::command();
    collect_command_names(&root_cmd, String::new(), &mut commands);
    commands.extend(["exit".into(), "help".into()]);
    let helper = ReplHelper { commands };
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    let history_file = history_path();
    if history_file.exists() {
        rl.load_history(&history_file)?;
    } else {
        println!("(No previous history)");
    }

    println!("Type 'help' or 'exit'.");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                match handle_input(application, line).await {
                    Ok(true) => break, // exit command
                    Ok(false) => {}    // continue REPL
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => return Err(err.into()),
        }
    }

    rl.save_history(&history_file)?;
    Ok(())
}

/// Returns Ok(true) if the user typed "exit", otherwise Ok(false)
async fn handle_input(application: &mut Application, line: String) -> anyhow::Result<bool> {
    let args = shell_words::split(&line)?;
    if args.is_empty() {
        return Ok(false);
    }

    if args[0] == "exit" {
        return Ok(true);
    } else if args[0] == "help" {
        println!("Available commands:");
        crate::cli::ReplCli::command().print_help()?;
        println!();
        return Ok(false);
    }

    match crate::cli::ReplCli::try_parse_from(
        std::iter::once("repl").chain(args.iter().map(String::as_str)),
    ) {
        Ok(parsed) => {
            if let Some(cmd) = parsed.command {
                crate::cli::handle_command(cmd, application).await?;
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(false)
}
