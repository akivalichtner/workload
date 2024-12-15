use std::net::TcpStream;

use crate::database::database_error::DatabaseError;

use super::column_type::ColumnType;
use super::type_stream::TypeStream;

pub struct CommandStream {
    stream: TypeStream,
}

impl CommandStream {
    pub fn new(tcp_stream: TcpStream) -> CommandStream {
        CommandStream { stream: TypeStream::new(tcp_stream) }
    }

    pub fn write_command(&mut self, command: &DriverProtocolCommand) -> Result<(), DatabaseError> {
        self.stream.write_u8(&CommandStream::get_op_code(&command))?;
        match command {
            DriverProtocolCommand::Authenticate { user, password } => {
                self.stream.write_string(user)?;
                self.stream.write_string(password)?;
                Ok(())
            }
            DriverProtocolCommand::Commit => Ok(()),
            DriverProtocolCommand::Execute { sql } => self.stream.write_string(sql),
            DriverProtocolCommand::Fail => Ok(()),
            DriverProtocolCommand::Fetch { fetch_size } => self.stream.write_u64(fetch_size),
            DriverProtocolCommand::GetUpdateCount => Ok(()),
            DriverProtocolCommand::Pass => Ok(()),
            DriverProtocolCommand::Ready => Ok(()),
            DriverProtocolCommand::ResultSetMetadata { column_names, column_types } => todo!(),
            DriverProtocolCommand::Row => Ok(()),
            DriverProtocolCommand::String { value } => self.stream.write_string(value),
            DriverProtocolCommand::Type { value } => self.stream.write_type(value),
            DriverProtocolCommand::U8 { value } => self.stream.write_u8(value),
            DriverProtocolCommand::U64 { value } => self.stream.write_u64(value),
        }
    }

    pub fn read_command(&self) -> Result<DriverProtocolCommand, DatabaseError> {
        todo!()
    }

    fn get_op_code(command: &DriverProtocolCommand) -> u8 {
        match command {
            DriverProtocolCommand::Authenticate { user: _, password: _ } => 1,
            DriverProtocolCommand::Commit => 2,
            DriverProtocolCommand::GetUpdateCount => 3,
            DriverProtocolCommand::Execute { sql: _ } => 4,
            DriverProtocolCommand::Fail => 5,
            DriverProtocolCommand::Fetch { fetch_size: _ } => 6,
            DriverProtocolCommand::Pass => 7,
            DriverProtocolCommand::Ready => 8,
            DriverProtocolCommand::ResultSetMetadata { column_names: _, column_types: _ } => 9,
            DriverProtocolCommand::Row => 10,
            DriverProtocolCommand::String { value: _ } => 11,
            DriverProtocolCommand::Type { value: _ } => 12,
            DriverProtocolCommand::U8 { value: _ } => 13,
            DriverProtocolCommand::U64 { value: _ } => 14,
        }
    }

    pub fn read_result_set_metadata(&mut self) -> Result<DriverProtocolCommand, DatabaseError> {
        let count = self.stream.read_u8()?;
        let mut column_names = Vec::new();
        let mut column_types = Vec::new();
        for _ in 1..count {
            column_names.push(self.stream.read_string()?);
            column_types.push(self.stream.read_type()?);
        }
        Ok(DriverProtocolCommand::ResultSetMetadata { column_names, column_types })
    }

}

pub enum DriverProtocolCommand<'a> {
    Authenticate { user: &'a str, password: &'a str },
    Commit,
    Execute { sql: &'a str },
    Fail,
    Fetch { fetch_size: u64 },
    GetUpdateCount,
    Pass,
    Ready,
    ResultSetMetadata { column_names: Vec<String>, column_types: Vec<ColumnType> },
    Row,
    String { value: String },
    Type { value: ColumnType },
    U8 { value: u8 },
    U64 { value: u64 },
}
