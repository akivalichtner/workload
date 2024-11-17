use std::{
    fmt::{self},
    net::TcpStream,
};

struct DataSource {
    url: String,
    port: u16,
    user: String,
    password: String,
}

impl DataSource {
    fn new(url: &str, port: u16, user: &str, password: &str) -> DataSource {
        DataSource {
            url: String::from(url),
            port,
            user: String::from(user),
            password: String::from(password),
        }
    }

    fn get_connection(&self) -> Result<Connection, DatabaseError> {
        let mut connection = Connection { tcp_stream: None };
        match connection.connect(&self.url, self.port, &self.user, &self.password) {
            Ok(_) => Ok(connection),
            Err(error) => Err(error),
        }
    }
}

enum DatabaseError {
    ConnectToListenerFailed,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self)
    }
}

struct Connection {
    tcp_stream: Option<TcpStream>,
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
                self.tcp_stream = Some(tcp_stream);
                return Ok(());
            }
            Err(error) => return Err(DatabaseError::ConnectToListenerFailed),
        }
    }

    fn create_statement(&self) -> Statement {
        Statement {}
    }

    fn commit(&self) {
        todo!()
    }
}

struct Statement {}

impl Statement {
    fn execute_query(&self, sql: &str) -> Result<ResultSet, DatabaseError> {
        Ok(ResultSet {})
    }

    fn execute_update(&self, sql: &str) -> Result<u64, DatabaseError> {
        todo!()
    }
}

struct ResultSet {}

impl ResultSet {
    fn has_next(&self) -> bool {
        todo!()
    }

    fn next(&self) {
        todo!()
    }

    fn get_string(&self, column: &str) {
        todo!()
    }
}

fn main() {
    let data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
    match data_source.get_connection() {
        Ok(connection) => {
            let statement = connection.create_statement();
            match statement.execute_update("INSERT INTO t (c) VALUES (1)") {
                Ok(row_count) => {
                    println!("Updated {} rows", row_count);
                }
                Err(database_error) => {
                    println!("Update failed: {}", database_error);
                }
            }
            match statement.execute_query("SELECT c FROM t") {
                Ok(result_set) => {
                    while result_set.has_next() {
                        result_set.next();
                        result_set.get_string("c");
                    }
                    connection.commit();
                }
                Err(database_error) => {
                    println!("Query failed: {}", database_error);
                }
            }
        }
        Err(error) => {
            println!("Error connection to database: {}", error);
        }
    }
}
