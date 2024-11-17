
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
            port,
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
    
    fn commit(&self) {
        todo!()
    }
}

struct Statement {
}

impl Statement {
    fn execute_query(&self, sql: &str) -> ResultSet {
        ResultSet{}
    }

    fn execute_update(&self, sql: &str) -> u64 {
        todo!()
    }
}

struct ResultSet {

}

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
    let connection = data_source.get_connection();
    let statement = connection.create_statement();
    let _rows = statement.execute_update("INSERT INTO t (c) VALUES (1)");
    let result_set = statement.execute_query("SELECT c FROM t");
    while result_set.has_next() {
        result_set.next();
        result_set.get_string("c");
    }
    connection.commit();
}
