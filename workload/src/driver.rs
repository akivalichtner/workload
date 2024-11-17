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
}
enum DriverProtocolCommand<'a> {
    Authenticate { user: &'a str, password: &'a str },
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
                self.authenticate(user, password)?
            }
            Err(error) => return Err(DatabaseError::ConnectToListenerFailed),
        }
    }

    pub fn create_statement(&self) -> Statement {
        Statement {}
    }

    pub fn commit(&self) {
        todo!()
    }

    fn authenticate(&self, user: &str, password: &str) -> Result<(), DatabaseError> {
        self.driver_protocol_stream
            .write(DriverProtocolCommand::Authenticate { user, password });
        
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
}
