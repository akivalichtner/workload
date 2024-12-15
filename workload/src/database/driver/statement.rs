use super::{
    command_stream::{Command, CommandStream},
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

    pub fn execute_query(&mut self, sql: &str) -> Result<ResultSet, DatabaseError> {
        self.command_stream.write_command(&Command::Execute { sql })?;
        match self.command_stream.read_command() {
            Ok(Command::ResultSetMetadata {
                column_names,
                column_types,
            }) => Ok(ResultSet::new(&mut self.command_stream, column_names, column_types)),
            Ok(_) => Err(DatabaseError::ProtocolViolation),
            Err(err) => Err(err),
        }
    }

    pub fn get_update_count(&mut self) -> Result<u64, DatabaseError> {
        self.command_stream.write_command(&Command::GetUpdateCount)?;
        match self.command_stream.read_command() {
            Ok(Command::U64 { value }) => Ok(value),
            Ok(_) => Err(DatabaseError::ProtocolViolation),
            Err(err) => Err(err),
        }
    }

    pub fn execute_update(&mut self, sql: &str) -> Result<u64, DatabaseError> {
        self.command_stream.write_command(&Command::Execute { sql })?;
        match self.command_stream.read_command() {
            Ok(Command::Ready) => self.get_update_count(),
            Ok(_) => Err(DatabaseError::ProtocolViolation),
            Err(err) => Err(err),
        }
    }
}
