use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    AuthenticationFailed,
    NetworkError,
    ConnectToListenerFailed,
    IllegalState,
    ProtocolViolation,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}
