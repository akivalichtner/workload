
use DatabaseError::NotImplemented;

pub fn connect(_connect_str: &str) -> Result<Connection, DatabaseError> {
    Err(NotImplemented)
}

pub struct Connection {

}

impl Connection {

    pub fn create_statement() -> Result<Statement, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    pub fn close() -> Result<(), DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }
}

pub struct Statement {

}

impl Statement {

    pub fn execute_query(_query: &str) -> Result<ResultSet, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    pub fn close() -> Result<(), DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }
}

pub struct ResultSet {

}

impl ResultSet {

    pub fn has_next() -> Result<bool, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    pub fn next() -> Result<(), DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

    pub fn get_str(_column: u8) -> Result<String, DatabaseError> {
        Err(DatabaseError::NotImplemented)
    }

}

pub enum DatabaseError {
    ConnectionRefused,
    NotImplemented
}
