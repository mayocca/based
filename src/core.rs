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
}

pub trait Driver {
    fn open(&self, url: &str) -> Result<Box<dyn Driver>, Box<dyn Error>>;
    fn create_database(&self, database: &mut Database) -> Result<(), Box<dyn Error>>;
    fn drop_database(&self, database: &mut Database) -> Result<(), Box<dyn Error>>;
    fn migration_table_exists(&self, database: &mut Database) -> Result<bool, Box<dyn Error>>;
    fn create_migrations_table(&self, database: &mut Database) -> Result<(), Box<dyn Error>>;
}
