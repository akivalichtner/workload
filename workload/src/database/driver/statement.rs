use crate::database::database_error::DatabaseError;

use super::{protocol_stream::{DriverProtocolCommand, DriverProtocolStream}, result_set::ResultSet};

pub struct Statement<'a> {
    driver_protocol_stream: &'a Option<DriverProtocolStream>,
}

impl<'a> Statement<'a> {
    pub fn new(driver_protocol_stream: &Option<DriverProtocolStream>) -> Statement {
        Statement { driver_protocol_stream }
    }

    pub fn execute_query(&self, _sql: &str) -> Result<ResultSet, DatabaseError> {
        Ok(ResultSet {})
    }

    pub fn execute_update(&mut self, sql: &str) -> Result<u64, DatabaseError> {
        if let Some(stream) = &mut self.driver_protocol_stream {
            match stream.write_command(&DriverProtocolCommand::Execute{ sql }) {
                Ok(()) => {
                    match stream.read() {
                        Ok(DriverProtocolCommand::Executed{ rows }) => Ok(rows),
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
