use super::{
    command_stream::{DriverProtocolCommand, CommandStream},
    statement::Statement,
};
use crate::database::database_error::DatabaseError;
use std::net::TcpStream;

pub struct Connection {
    command_stream: Option<CommandStream>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            command_stream: None,
        }
    }

    pub fn connect(&mut self, url: &str, port: u16, user: &str, password: &str) -> Result<(), DatabaseError> {
        match TcpStream::connect(format!("{}:{}", url, port)) {
            Ok(tcp_stream) => {
                self.command_stream = Some(CommandStream::new(tcp_stream));
                self.authenticate(user, password)
            }
            Err(_) => Err(DatabaseError::ConnectToListenerFailed),
        }
    }

    pub fn create_statement(&mut self) -> Statement {
        Statement::new(&mut self.command_stream)
    }

    pub fn commit(&mut self) -> Result<(), DatabaseError> {
        if let Some(stream) = &mut self.command_stream {
            match stream.write_command(&DriverProtocolCommand::Commit) {
                Ok(()) => match stream.read_command() {
                    Ok(DriverProtocolCommand::Pass) => Ok(()),
                    Ok(_) => Err(DatabaseError::ProtocolViolation),
                    Err(database_error) => Err(database_error),
                },
                Err(database_error) => Err(database_error),
            }
        } else {
            Err(DatabaseError::IllegalState)
        }
    }

    fn authenticate(&mut self, user: &str, password: &str) -> Result<(), DatabaseError> {
        if let Some(stream) = &mut self.command_stream {
            match stream.write_command(&DriverProtocolCommand::Authenticate { user, password }) {
                Ok(()) => match stream.read_command() {
                    Ok(DriverProtocolCommand::Pass) => Ok(()),
                    Ok(DriverProtocolCommand::Fail) => Err(DatabaseError::AuthenticationFailed),
                    Ok(_) => Err(DatabaseError::ProtocolViolation),
                    Err(database_error) => Err(database_error),
                },
                Err(database_error) => Err(database_error),
            }
        } else {
            Err(DatabaseError::IllegalState)
        }
    }
}
