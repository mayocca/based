use super::Connection;
use postgres::Client;
use std::error::Error;

pub struct PostgresConnection {
    client: Client,
}

impl PostgresConnection {
    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let config = format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname
        );

        let client = Client::connect(&config, postgres::NoTls)?;

        println!("Connected to Postgres");

        Ok(Self { client })
    }
}

impl Connection for PostgresConnection {
    fn create_migrations_table(&mut self) -> Result<(), Box<dyn Error>> {
        self.client
            .batch_execute(
                "create table if not exists public.migrations (
                version text not null primary key,
                statements text[],
                name text
            )",
            )
            .map_err(|e| e.into())
    }

    fn drop_migrations_table(&mut self) -> Result<(), Box<dyn Error>> {
        self.client
            .batch_execute("drop table if exists migrations")
            .map_err(|e| e.into())
    }
}
