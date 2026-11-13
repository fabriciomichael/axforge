use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "axforge",
    version,
    about = "CLI project generator for Rust web applications using Axum and Tera"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Axum web application
    New {
        /// Name of the project
        name: String,
    },
}
