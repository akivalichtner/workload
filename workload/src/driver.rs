use std::fmt;
use std::io::Write;
use std::net::TcpStream;
pub struct DataSource {
    url: String,
    port: u16,
    user: String,
    password: String,
}

impl DataSource {
    pub fn new(url: &str, port: u16, user: &str, password: &str) -> DataSource {
        DataSource {
            url: String::from(url),
            port,
            user: String::from(user),
            password: String::from(password),
        }
    }

    pub fn get_connection(&self) -> Result<Connection, DatabaseError> {
        let mut connection = Connection {
            driver_protocol_stream: None,
        };
        match connection.connect(&self.url, self.port, &self.user, &self.password) {
            Ok(_) => Ok(connection),
            Err(error) => Err(error),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}

struct DriverProtocolStream {
    tcp_stream: TcpStream,
}

impl DriverProtocolStream {
    fn new(tcp_stream: TcpStream) -> DriverProtocolStream {
        DriverProtocolStream { tcp_stream }
    }

    fn write_command(&mut self, command: &DriverProtocolCommand) -> Result<(), DatabaseError> {
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

    fn read(&self) -> Result<DriverProtocolCommand, DatabaseError> {
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
    
    fn write_u8(&mut self, value: u8) -> Result<(), DatabaseError>  {
        match DriverProtocolStream::write(&mut self.tcp_stream, &[value]) {
            Ok(_) => { Ok(()) }
            Err(_) => Err(DatabaseError::NetworkError)
        }
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

enum DriverProtocolCommand<'a> {
    Authenticate { user: &'a str, password: &'a str },
    Commit,
    Execute{ sql: &'a str },
    Executed{ rows: u64 },
    Fail,
    Pass,
}

pub struct Connection {
    driver_protocol_stream: Option<DriverProtocolStream>,
}

impl Connection {
    fn connect(
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

pub struct Statement<'a> {
    driver_protocol_stream: &'a Option<DriverProtocolStream>,
}

impl<'a> Statement<'a> {
    fn new(driver_protocol_stream: &'a Option<DriverProtocolStream>) -> Statement {
        Statement { driver_protocol_stream }
    }

    pub fn execute_query(&self, _sql: &str) -> Result<ResultSet, DatabaseError> {
        Ok(ResultSet {})
    }

    pub fn execute_update(&mut self, sql: &str) -> Result<u64, DatabaseError> {
        if let Some(stream) = &mut self.driver_protocol_stream {
            match &stream.write_command(&DriverProtocolCommand::Execute{ sql }) {
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
pub struct ResultSet {}

impl ResultSet {
    pub fn has_next(&self) -> bool {
        todo!()
    }

    pub fn next(&self) {
        todo!()
    }

    pub fn get_string(&self, _column: &str) {
        todo!()
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    AuthenticationFailed,
    NetworkError,
    ConnectToListenerFailed,
    IllegalState,
    ProtocolViolation,
}
