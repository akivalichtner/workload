
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
}

fn main() {
    let data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
    let _connection = data_source.get_connection();
}
