use super::connection::Connection;
use crate::database::database_error::DatabaseError;

pub struct DataSource {
    url: String,
    port: u16,
    user: String,
    password: String,
}

impl DataSource {
    pub fn new(url: &str, port: u16, user: &str, password: &str) -> DataSource {
        DataSource {
            url: String::from(url),
            port,
            user: String::from(user),
            password: String::from(password),
        }
    }

    pub fn get_connection(&self) -> Result<Connection, DatabaseError> {
        let mut connection = Connection::new();
        match connection.connect(&self.url, self.port, &self.user, &self.password) {
            Ok(_) => Ok(connection),
            Err(error) => Err(error),
        }
    }
}
