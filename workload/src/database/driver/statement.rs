use super::{
    command_stream::{CommandStream, DriverProtocolCommand},
    result_set::ResultSet,
};
use crate::database::database_error::DatabaseError;

pub struct Statement<'a> {
    command_stream: &'a mut CommandStream,
}

impl<'a> Statement<'a> {
    pub fn new(command_stream: &mut CommandStream) -> Statement {
        Statement { command_stream }
    }

    pub fn execute_query2(&mut self, sql: &str) -> Result<ResultSet, DatabaseError> {
        todo!()
    }

    pub fn execute_query(&mut self, sql: &str) -> Result<ResultSet, DatabaseError> {
        match self.command_stream.write_command(&DriverProtocolCommand::Execute { sql }) {
            Ok(()) => match self.command_stream.read_command() {
                Ok(DriverProtocolCommand::Ready) => {
                    let mut result_set = ResultSet::new(&mut self.command_stream);
                    result_set.read_metadata()?;
                    Ok(result_set)
                }
                Ok(_) => Err(DatabaseError::ProtocolViolation),
                Err(database_error) => Err(database_error),
            },
            Err(database_error) => Err(database_error),
        }
    }

    pub fn get_update_count(&mut self) -> Result<u64, DatabaseError> {
        match self.command_stream.write_command(&DriverProtocolCommand::GetUpdateCount) {
            Ok(()) => match self.command_stream.read_command() {
                Ok(DriverProtocolCommand::U64 { value }) => Ok(value),
                Ok(_) => Err(DatabaseError::ProtocolViolation),
                Err(database_error) => Err(database_error),
            },
            Err(database_error) => Err(database_error),
        }
    }

    pub fn execute_update(&mut self, sql: &str) -> Result<u64, DatabaseError> {
        match self.command_stream.write_command(&DriverProtocolCommand::Execute { sql }) {
            Ok(()) => match self.command_stream.read_command() {
                Ok(DriverProtocolCommand::Ready) => self.get_update_count(),
                Ok(_) => Err(DatabaseError::ProtocolViolation),
                Err(database_error) => Err(database_error),
            },
            Err(database_error) => Err(database_error),
        }
    }
}
