use std::io::Write;
use std::net::TcpStream;

use crate::database::database_error::DatabaseError;

pub struct DriverProtocolStream {
    tcp_stream: TcpStream,
}

impl DriverProtocolStream {
    pub fn new(tcp_stream: TcpStream) -> DriverProtocolStream {
        DriverProtocolStream { tcp_stream }
    }

    pub fn write_command(&mut self, command: &DriverProtocolCommand) -> Result<(), DatabaseError> {
        self.write_u8(DriverProtocolStream::get_op_code(&command))?;
        match command {
            DriverProtocolCommand::Authenticate { user, password } => {
                self.write_string(user)?;
                self.write_string(password)?;
                Ok(())
            }
            DriverProtocolCommand::Execute { sql } => self.write_string(sql),
            DriverProtocolCommand::GetUpdateCount => Ok(()),
            DriverProtocolCommand::Commit => Ok(()),
            DriverProtocolCommand::Fail => Ok(()),
            DriverProtocolCommand::Fetch{ fetch_size } => self.write_u64(fetch_size),
            DriverProtocolCommand::Pass => Ok(()),
            DriverProtocolCommand::U64 { value } => self.write_u64(value),
            DriverProtocolCommand::Ready => Ok(()),
            DriverProtocolCommand::Row => Ok(()),
        }
    }

    pub fn read_command(&self) -> Result<DriverProtocolCommand, DatabaseError> {
        todo!()
    }

    fn get_op_code(command: &DriverProtocolCommand) -> u8 {
        match command {
            DriverProtocolCommand::Authenticate {
                user: _,
                password: _,
            } => 1,
            DriverProtocolCommand::Commit => 2,
            DriverProtocolCommand::Execute { sql: _ } => 3,
            DriverProtocolCommand::GetUpdateCount => 4,
            DriverProtocolCommand::Fail => 5,
            DriverProtocolCommand::Fetch { fetch_size: _ }=> 6,
            DriverProtocolCommand::Pass => 7,
            DriverProtocolCommand::U64 { value: _ } => 8,
            DriverProtocolCommand::Ready => 9,
            DriverProtocolCommand::Row => 10,
        }
    }

    fn write_u8(&mut self, value: u8) -> Result<(), DatabaseError> {
        match DriverProtocolStream::write(&mut self.tcp_stream, &[value]) {
            Ok(_) => Ok(()),
            Err(_) => Err(DatabaseError::NetworkError),
        }
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
    Ready,
    Row,
    Fail,
    Fetch { fetch_size: u64 },
    Pass,
    GetUpdateCount,
    U64 { value: u64 },
}
