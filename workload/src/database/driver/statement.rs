use super::{
    protocol_stream::{DriverProtocolCommand, DriverProtocolStream},
    result_set::ResultSet,
};
use crate::database::database_error::DatabaseError;

pub struct Statement<'a> {
    driver_protocol_stream: &'a mut Option<DriverProtocolStream>,
}

impl<'a> Statement<'a> {
    pub fn new(driver_protocol_stream: &mut Option<DriverProtocolStream>) -> Statement {
        Statement { driver_protocol_stream }
    }

    pub fn execute_query(&mut self, sql: &str) -> Result<ResultSet, DatabaseError> {
        if let Some(ref mut stream) = &mut self.driver_protocol_stream {
            match stream.write_command(&DriverProtocolCommand::Execute { sql }) {
                Ok(()) => match stream.read_command() {
                    Ok(DriverProtocolCommand::Ready) => {
                        let mut result_set = ResultSet::new(stream);
                        result_set.read_metadata()?;
                        Ok(result_set)
                    }
                    Ok(_) => Err(DatabaseError::ProtocolViolation),
                    Err(database_error) => Err(database_error),
                },
                Err(database_error) => Err(database_error),
            }
        } else {
            Err(DatabaseError::IllegalState)
        }
    }

    pub fn get_update_count(&mut self) -> Result<u64, DatabaseError> {
        if let Some(ref mut stream) = &mut self.driver_protocol_stream {
            match stream.write_command(&DriverProtocolCommand::GetUpdateCount) {
                Ok(()) => match stream.read_command() {
                    Ok(DriverProtocolCommand::U64 { value }) => Ok(value),
                    Ok(_) => Err(DatabaseError::ProtocolViolation),
                    Err(database_error) => Err(database_error),
                },
                Err(database_error) => Err(database_error),
            }
        } else {
            Err(DatabaseError::IllegalState)
        }
    }

    pub fn execute_update(&mut self, sql: &str) -> Result<u64, DatabaseError> {
        if let Some(ref mut stream) = &mut self.driver_protocol_stream {
            match stream.write_command(&DriverProtocolCommand::Execute { sql }) {
                Ok(()) => match stream.read_command() {
                    Ok(DriverProtocolCommand::Ready) => self.get_update_count(),
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
