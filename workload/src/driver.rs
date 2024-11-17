use std::fmt;
use std::net;
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
    tcp_stream: net::TcpStream,
}

impl DriverProtocolStream {
    fn new(tcp_stream: net::TcpStream) -> DriverProtocolStream {
        DriverProtocolStream { tcp_stream }
    }

    fn write(&self, command: DriverProtocolCommand) -> Result<(), DatabaseError> {
        todo!()
    }

    fn read(&self) -> Result<DriverProtocolCommand, DatabaseError> {
        todo!()
    }
}
enum DriverProtocolCommand<'a> {
    Authenticate { user: &'a str, password: &'a str },
    Fail,
    Pass,
    Commit,
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
        match net::TcpStream::connect(format!("{}:{}", url, port)) {
            Ok(tcp_stream) => {
                self.driver_protocol_stream = Some(DriverProtocolStream::new(tcp_stream));
                self.authenticate(user, password)
            }
            Err(_) => Err(DatabaseError::ConnectToListenerFailed),
        }
    }

    pub fn create_statement(&self) -> Statement {
        Statement {}
    }

    pub fn commit(&self) -> Result<(), DatabaseError> {
        if let Some(stream) = &self.driver_protocol_stream {
            match stream.write(DriverProtocolCommand::Commit) {
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

    fn authenticate(&self, user: &str, password: &str) -> Result<(), DatabaseError> {
        if let Some(stream) = &self.driver_protocol_stream {
            match stream.write(DriverProtocolCommand::Authenticate { user, password }) {
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

pub struct Statement {}

impl Statement {
    pub fn execute_query(&self, sql: &str) -> Result<ResultSet, DatabaseError> {
        Ok(ResultSet {})
    }

    pub fn execute_update(&self, sql: &str) -> Result<u64, DatabaseError> {
        todo!()
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

    pub fn get_string(&self, column: &str) {
        todo!()
    }
}
pub enum DatabaseError {
    ConnectToListenerFailed,
    AuthenticationFailed,
    ProtocolViolation,
    IllegalState,
}
