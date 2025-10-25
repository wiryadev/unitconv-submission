mod cli;
mod conversion;
mod history;
mod unit;

use crate::history::{add_and_save_entry, display_history};
use clap::{CommandFactory, FromArgMatches};
use cli::{Cli, Commands, display_all_units};
use conversion::convert_value;
use regex;
use std::process;

fn main() {
    let cmd = Cli::command();

    match cmd.try_get_matches_from(std::env::args_os()) {
        Ok(matches) => {
            let cli =
                Cli::from_arg_matches(&matches).expect("Failed to create Cli struct from matches");
            match cli.command {
                Commands::Convert { from, to, value } => match convert_value(from, to, value) {
                    Ok(result) => {
                        println!("{}", result.display_string);

                        // Save the structured history entry
                        if let Err(e) = add_and_save_entry(result.history_entry) {
                            eprintln!("Warning: Failed to save history: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                },
                Commands::List => {
                    println!("{}", display_all_units());
                }
                Commands::History => {
                    if let Err(e) = display_history() {
                        eprintln!("Error: Failed to read history: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            let err_string = e.to_string();
            let pattern = r"(?i)--(from|to).*?: invalid unit: (\S+)";

            if let Some(captures) = regex::Regex::new(pattern).unwrap().captures(&err_string) {
                let arg_name = captures.get(1).map_or("", |m| m.as_str());
                let invalid_value = captures.get(2).map_or("", |m| m.as_str());

                let unit_type = if arg_name == "from" {
                    "original"
                } else {
                    "target"
                };

                eprintln!("[ERROR] invalid {} unit: {}", unit_type, invalid_value);
                process::exit(1);
            }

            e.print().expect("Failed to print clap error");
            process::exit(1);
        }
    }
}
