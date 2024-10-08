use std::io::Error;
use std::net::TcpStream;
pub struct Driver {}

impl Driver {
    pub fn connect(host: &str, port: u16) -> Result<Connection, DatabaseError> {
        let mut connection = Connection {
            host: String::from(host),
            port: port,
            tcp_stream: None,
            error: None,
        };
        connection.connect();
        if connection.connected() {
            Ok(connection)
        } else {
            Err(DatabaseError::ConnectionFailed)
        }
    }
}

pub struct Connection {
    host: String,
    port: u16,
    tcp_stream: Option<TcpStream>,
    error: Option<Error>,
}

impl Connection {
    fn connect(&mut self) -> () {
        let address = format!("{}:{}", &self.host, self.port);
        match TcpStream::connect(&address) {
            Ok(tcp_stream) => self.tcp_stream = Some(tcp_stream),
            Err(error) => self.error = Some(error),
        }
    }

    fn connected(&self) -> bool {
        match self.tcp_stream {
            Some(_) => true,
            None => false
        }
    }

    fn create_statement(&mut self) -> Result<Statement, DatabaseError> {
        Ok(Statement { connection: self })
    }

    fn close(&mut self) -> Result<(), DatabaseError> {
        todo!();
    }

}

pub struct Statement<'a> {
    connection: &'a Connection
}

impl Statement<'_> {

    fn close(&mut self) -> Result<(), DatabaseError> {
        todo!();
    }

}

pub enum DatabaseError {
    ConnectionFailed,
    NotImplemented,
}
