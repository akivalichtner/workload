use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    AuthenticationFailed,
    Defect,
    NetworkError,
    ConnectToListenerFailed,
    IllegalState,
    ProtocolViolation,
    NoSuchColumn,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}
