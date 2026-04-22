use postgres::error::SqlState;


pub enum Error {
    PostgresError(Option<SqlState>)
}
pub trait Database {
    fn init_if_uninitialized(&mut self) -> Result<(), Error>;
}