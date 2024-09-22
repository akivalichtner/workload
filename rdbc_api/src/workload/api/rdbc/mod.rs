
pub fn connect(_connect_str: &str) -> Result<Connection, DatabaseError> {
    Result::Err(DatabaseError::ConnectionRefused)
}

pub struct Connection {

}

pub enum DatabaseError {
    ConnectionRefused
}
