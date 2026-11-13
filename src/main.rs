mod cli;
mod commands;
mod generator;
mod validation;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            if let Err(error) = validation::project_name::validate(&name) {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }

            commands::new::execute(name);
        }
    }
}
