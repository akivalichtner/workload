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
            DriverProtocolCommand::Authenticate{user, password} => {
                self.write_string(user)?;
                self.write_string(password)?;
                Ok(())
            }
            DriverProtocolCommand::Execute { sql } => {
                self.write_string(sql)?;
                Ok(())
            }
            DriverProtocolCommand::Executed { rows: _ } => {
                todo!()
            }
            DriverProtocolCommand::Commit => {
                todo!()
            }
            DriverProtocolCommand::Fail => {
                todo!()
            }
            DriverProtocolCommand::Pass => {
                todo!()
            }
        }
    }

    pub fn read(&self) -> Result<DriverProtocolCommand, DatabaseError> {
        todo!()
    }

    fn get_op_code(command: &DriverProtocolCommand) -> u8 {
        match command {
            DriverProtocolCommand::Authenticate { user: _ , password: _ } => 1,
            DriverProtocolCommand::Commit => 2,
            DriverProtocolCommand::Execute { sql: _ } => 3,
            DriverProtocolCommand::Executed { rows: _ } => 4,
            DriverProtocolCommand::Fail => 5,
            DriverProtocolCommand::Pass => 6,
        }
    }
    
    pub fn write_u8(&mut self, value: u8) -> Result<(), DatabaseError>  {
        match DriverProtocolStream::write(&mut self.tcp_stream, &[value]) {
            Ok(_) => { Ok(()) }
            Err(_) => Err(DatabaseError::NetworkError)
        }
    }
    
    pub fn write_string(&self, _user: &str) -> Result<(), DatabaseError> {
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
    Execute{ sql: &'a str },
    Executed{ rows: u64 },
    Fail,
    Pass,
}
