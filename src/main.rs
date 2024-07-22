mod db;

use clap::Parser;
use db::{Connection, PostgresConnection};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "postgres")]
    db_type: String,

    #[arg(long)]
    host: String,

    #[arg(long)]
    port: u16,

    #[arg(long)]
    user: String,

    #[arg(long)]
    password: String,

    #[arg(short, long)]
    database: String,

    #[arg(long, default_value = "migrations")]
    schema: String,

    #[arg(long, default_value = "migrations")]
    migrations_dir: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut db = match args.db_type.as_str() {
        "postgres" => Box::new(PostgresConnection::new(
            &args.host,
            args.port,
            &args.user,
            &args.password,
            &args.database,
        )?),
        _ => return Err("Unsupported database type".into()),
    };

    db.create_migrations_table()?;

    Ok(())
}
