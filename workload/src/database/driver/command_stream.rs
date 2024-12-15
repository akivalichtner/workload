use std::io::Write;
use std::net::TcpStream;

use crate::database::database_error::DatabaseError;

use super::column_type::ColumnType;

pub struct CommandStream {
    tcp_stream: TcpStream,
}

impl CommandStream {
    pub fn new(tcp_stream: TcpStream) -> CommandStream {
        CommandStream { tcp_stream }
    }

    pub fn write_command(&mut self, command: &DriverProtocolCommand) -> Result<(), DatabaseError> {
        self.write_u8(&CommandStream::get_op_code(&command))?;
        match command {
            DriverProtocolCommand::Authenticate { user, password } => {
                self.write_string(user)?;
                self.write_string(password)?;
                Ok(())
            }
            DriverProtocolCommand::Commit => Ok(()),
            DriverProtocolCommand::Execute { sql } => self.write_string(sql),
            DriverProtocolCommand::Fail => Ok(()),
            DriverProtocolCommand::Fetch { fetch_size } => self.write_u64(fetch_size),
            DriverProtocolCommand::GetUpdateCount => Ok(()),
            DriverProtocolCommand::Pass => Ok(()),
            DriverProtocolCommand::Ready => Ok(()),
            DriverProtocolCommand::Row => Ok(()),
            DriverProtocolCommand::String { value } => self.write_string(value),
            DriverProtocolCommand::Type { value } => self.write_type(value),
            DriverProtocolCommand::U8 { value } => self.write_u8(value),
            DriverProtocolCommand::U64 { value } => self.write_u64(value),
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
            DriverProtocolCommand::Row => 9,
            DriverProtocolCommand::String { value: _ } => 10,
            DriverProtocolCommand::Type { value: _ } => 11,
            DriverProtocolCommand::U8 { value: _ } => 12,
            DriverProtocolCommand::U64 { value: _ } => 13,
        }
    }

    fn write_type(&mut self, value: &ColumnType) -> Result<(), DatabaseError> {
        todo!()
    }

    fn write_u8(&mut self, value: &u8) -> Result<(), DatabaseError> {
        todo!()
    }

    fn write_u64(&mut self, _value: &u64) -> Result<(), DatabaseError> {
        todo!()
    }

    fn write_string(&self, _user: &str) -> Result<(), DatabaseError> {
        todo!()
    }

    fn write(tcp_stream: &mut TcpStream, buf: &[u8]) -> Result<(), DatabaseError> {
        match tcp_stream.write(buf) {
            Ok(_) => Ok(()),
            Err(_) => {
                // FIXME wrap error source
                Err(DatabaseError::NetworkError)
            }
        }
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
    Row,
    String { value: String },
    Type { value: ColumnType },
    U8 { value: u8 },
    U64 { value: u64 },
}
