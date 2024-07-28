use clap::{Parser, Subcommand};

/// A framework-agnostic database migration tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The URL of the database
    #[arg(short, long)]
    pub url: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Create migrations table
    Init,
    /// Create a new migration
    #[command(arg_required_else_help = true)]
    Create {
        /// The name of the migration
        name: String,
    },
    /// Drop the database
    Drop,
    /// Execute all pending migrations
    Migrate,
}
