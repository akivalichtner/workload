
pub fn new_data_source() -> Box<dyn DataSource> {
    Box::new(DataSourceImpl{})
}

pub trait DataSource {

    fn get_connection<'a, 'b>(&'a mut self) -> Box<dyn Connection<'b>> where 'b: 'a;
}

struct DataSourceImpl {
}

impl DataSource for DataSourceImpl {
    fn get_connection<'a, 'b>(&'a mut self) -> Box<dyn Connection<'b>> where 'b: 'a {
        Box::new(ConnectionImpl{})
    }
}

pub trait Connection<'a> {

    fn create_statement(&mut self) -> Box<dyn Statement>;
}

struct ConnectionImpl {

}

impl<'a> Connection<'a> for ConnectionImpl {
    fn create_statement(&mut self) -> Box<dyn Statement> {
        Box::new(StatementImpl{})
    }
}

impl Drop for ConnectionImpl {
    fn drop(&mut self) {
        println!("drop");
    }
}

pub trait Statement {

}

struct StatementImpl {

}

impl Statement for StatementImpl {

}

fn main() {
    let mut data_source = new_data_source();
    { 
        println!("allocating connection 1");
        let _connection = data_source.get_connection();
    }
    { 
        println!("allocating connection 2");
        let _connection = data_source.get_connection();
    }
}
