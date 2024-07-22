use std::error::Error;

pub trait Connection {
    fn create_migrations_table(&mut self) -> Result<(), Box<dyn Error>>;
    fn drop_migrations_table(&mut self) -> Result<(), Box<dyn Error>>;
}
