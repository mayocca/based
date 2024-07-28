use postgres::Client;
use std::error::Error;

use crate::core::Driver;

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
