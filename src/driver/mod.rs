mod postgres;

use std::error::Error;

pub struct Database {
    pub url: String,
    pub migrations_directory: Vec<String>,
    pub migrations_table: String,
}

impl Database {
    pub fn new(url: &str) -> Database {
        Database {
            url: url.to_string(),
            migrations_directory: vec!["migrations".to_string()],
            migrations_table: "migrations".to_string(),
        }
    }

    pub fn create() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn driver() -> Result<Box<dyn Driver>, Box<dyn Error>> {
        todo!()
    }

    pub fn drop() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn migrations() -> Result<Vec<String>, Box<dyn Error>> {
        todo!()
    }

    pub fn migrate() -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn status() -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

/// The `Driver` trait defines the interface for database drivers.  It provides methods for
/// creating, dropping, and checking the existence of databases, migrations tables, and migrations.
/// Each database driver must implement this trait.
pub trait Driver {
    fn new(url: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn open(&self) -> Result<(), Box<dyn Error>>;
    fn database_exists(&self) -> Result<bool, Box<dyn Error>>;
    fn create_database(&mut self) -> Result<(), Box<dyn Error>>;
    fn drop_database(&self) -> Result<(), Box<dyn Error>>;
    fn migrations_table_exists(&self, database: &mut Database) -> Result<bool, Box<dyn Error>>;
    fn create_migrations_table(&self, database: &mut Database) -> Result<(), Box<dyn Error>>;
    fn select_migrations(&self, database: &mut Database) -> Result<Vec<String>, Box<dyn Error>>;
    fn insert_migration(
        &self,
        database: &mut Database,
        migration: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn delete_migration(
        &self,
        database: &mut Database,
        migration: &str,
    ) -> Result<(), Box<dyn Error>>;
}
