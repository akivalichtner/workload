
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
}

fn main() {
    let _data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
}
