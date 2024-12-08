use std::net::TcpStream;

use crate::database::database_error::DatabaseError;

use super::{protocol_stream::{DriverProtocolCommand, DriverProtocolStream}, statement::Statement};

pub struct Connection {
    driver_protocol_stream: Option<DriverProtocolStream>,
}

impl Connection {

    pub fn new() -> Connection {
        Connection{
            driver_protocol_stream: None,
        }
    }

    pub fn connect(
        &mut self,
        url: &str,
        port: u16,
        user: &str,
        password: &str,
    ) -> Result<(), DatabaseError> {
        match TcpStream::connect(format!("{}:{}", url, port)) {
            Ok(tcp_stream) => {
                self.driver_protocol_stream = Some(DriverProtocolStream::new(tcp_stream));
                self.authenticate(user, password)
            }
            Err(_) => Err(DatabaseError::ConnectToListenerFailed),
        }
    }

    pub fn create_statement(&self) -> Statement {
        Statement::new(&self.driver_protocol_stream)
    }

    pub fn commit(&mut self) -> Result<(), DatabaseError> {
        if let Some(stream) = &mut self.driver_protocol_stream {
            match stream.write_command(&DriverProtocolCommand::Commit) {
                Ok(()) => {
                    match stream.read() {
                        Ok(DriverProtocolCommand::Pass) => Ok(()),
                        Ok(_) => Err(DatabaseError::ProtocolViolation),
                        Err(database_error) => Err(database_error)
                    }        
                },
                Err(database_error) => Err(database_error)
            }
        } else {
            Err(DatabaseError::IllegalState)
        }
    }

    fn authenticate(&mut self, user: &str, password: &str) -> Result<(), DatabaseError> {
        if let Some(stream) = &mut self.driver_protocol_stream {
            match stream.write_command(&DriverProtocolCommand::Authenticate { user, password }) {
                Ok(()) => {
                    match stream.read() {
                        Ok(DriverProtocolCommand::Pass) => Ok(()),
                        Ok(DriverProtocolCommand::Fail) => Err(DatabaseError::AuthenticationFailed),
                        Ok(_) => Err(DatabaseError::ProtocolViolation),
                        Err(database_error) => Err(database_error)
                    }        
                },
                Err(database_error) => Err(database_error)
            }
        } else {
            Err(DatabaseError::IllegalState)
        }
    }
}
