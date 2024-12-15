use std::net::TcpStream;

use crate::database::database_error::DatabaseError;

use super::column_type::ColumnType;
use super::type_stream::TypeStream;

pub struct CommandStream {
    stream: TypeStream,
}

impl CommandStream {
    pub fn new(tcp_stream: TcpStream) -> CommandStream {
        CommandStream {
            stream: TypeStream::new(tcp_stream),
        }
    }

    pub fn write_command(&mut self, command: &Command) -> Result<(), DatabaseError> {
        self.stream.write_u8(&CommandStream::get_op_code(&command))?;
        match command {
            Command::Authenticate { user, password } => {
                self.stream.write_string(user)?;
                self.stream.write_string(password)?;
                Ok(())
            }
            Command::Commit => Ok(()),
            Command::Execute { sql } => self.stream.write_string(sql),
            Command::Fail => Ok(()),
            Command::Fetch { fetch_size } => self.stream.write_u64(fetch_size),
            Command::GetUpdateCount => Ok(()),
            Command::Pass => Ok(()),
            Command::Ready => Ok(()),
            Command::ResultSetMetadata {
                column_names,
                column_types,
            } => todo!(),
            Command::Row { values } => Ok(()),
            Command::String { value } => self.stream.write_string(value),
            Command::Type { value } => self.stream.write_type(value),
            Command::U8 { value } => self.stream.write_u8(value),
            Command::U64 { value } => self.stream.write_u64(value),
        }
    }

    pub fn read_command(&self) -> Result<Command, DatabaseError> {
        todo!()
    }

    fn get_op_code(command: &Command) -> u8 {
        match command {
            Command::Authenticate { user: _, password: _ } => 1,
            Command::Commit => 2,
            Command::GetUpdateCount => 3,
            Command::Execute { sql: _ } => 4,
            Command::Fail => 5,
            Command::Fetch { fetch_size: _ } => 6,
            Command::Pass => 7,
            Command::Ready => 8,
            Command::ResultSetMetadata {
                column_names: _,
                column_types: _,
            } => 9,
            Command::Row { values: _ } => 10,
            Command::String { value: _ } => 11,
            Command::Type { value: _ } => 12,
            Command::U8 { value: _ } => 13,
            Command::U64 { value: _ } => 14,
        }
    }

    pub fn read_result_set_metadata(&mut self) -> Result<Command, DatabaseError> {
        let count = self.stream.read_u8()?;
        let mut column_names = Vec::new();
        let mut column_types = Vec::new();
        for _ in 1..count {
            column_names.push(self.stream.read_string()?);
            column_types.push(self.stream.read_type()?);
        }
        Ok(Command::ResultSetMetadata {
            column_names,
            column_types,
        })
    }
}

pub enum Command<'a> {
    Authenticate {
        user: &'a str,
        password: &'a str,
    },
    Commit,
    Execute {
        sql: &'a str,
    },
    Fail,
    Fetch {
        fetch_size: u64,
    },
    GetUpdateCount,
    Pass,
    Ready,
    ResultSetMetadata {
        column_names: Vec<String>,
        column_types: Vec<ColumnType>,
    },
    Row { values: Vec<Vec<u8>> },
    String {
        value: String,
    },
    Type {
        value: ColumnType,
    },
    U8 {
        value: u8,
    },
    U64 {
        value: u64,
    },
}
