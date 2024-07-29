use postgres::Client;
use std::error::Error;

use super::{Database, Driver};

pub struct PostgresConnection {
    client: Client,
}

impl Driver for PostgresConnection {
    fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::connect(&url, postgres::NoTls)?;

        Ok(Self { client })
    }

    fn open(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn database_exists(&self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn create_database(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn drop_database(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn migrations_table_exists(&self, database: &mut Database) -> Result<bool, Box<dyn Error>> {
        todo!()
    }

    fn create_migrations_table(&self, database: &mut Database) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn select_migrations(&self, database: &mut Database) -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    fn insert_migration(
        &self,
        database: &mut Database,
        migration: &str,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn delete_migration(
        &self,
        database: &mut Database,
        migration: &str,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
