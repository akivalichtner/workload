
struct DataSource {
    url: String,
    port: u16,
    user: String,
    password: String
}

impl DataSource {
    fn new(url: &str, port: u16, user: &str, password: &str) -> DataSource {
        DataSource {
            url: String::from(url),
            port: port,
            user: String::from(user),
            password: String::from(password)
        }
    }

    fn get_connection(&self) -> Connection {
        let connection = Connection{};
        connection.connect(&self.url, self.port, &self.user, &self.password);
        connection
    }
}

struct Connection {

}

impl Connection {
    fn connect(&self, url: &str, port: u16, user: &str, password: &str) -> () {
        todo!()
    }

    fn create_statement(&self) -> Statement {
        Statement{
        }
    }
}

struct Statement {
}

impl Statement {
    fn execute_query(&self, sql: &str) -> ResultSet {
        ResultSet{}
    }
}

struct ResultSet {
    
}

fn main() {
    let data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
    let mut connection = data_source.get_connection();
    let statement = connection.create_statement();
    let _result_set = statement.execute_query("INSERT INTO t (c) VALUES (1)");
}
