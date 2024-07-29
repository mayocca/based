mod cli;
mod driver;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    match args.command {
        cli::Command::Create { name } => {
            println!("Creating migration: {name}");
        }
        cli::Command::Drop => {
            println!("Dropping database");
        }
        cli::Command::Init => {
            println!("Creating migrations table");
        }
        cli::Command::Migrate => {
            println!("Executing all pending migrations");
        }
    };
}
