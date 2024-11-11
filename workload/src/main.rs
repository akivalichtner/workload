
pub fn new_data_source() -> Box<dyn DataSource> {
    Box::new(DataSourceImpl{})
}

pub trait DataSource {

    fn get_connection<'a, 'b>(&'a mut self) -> Box<dyn Connection<'b>>;
}

struct DataSourceImpl {
}

impl DataSource for DataSourceImpl {
    fn get_connection<'a, 'b>(&'a mut self) -> Box<dyn Connection<'b>> {
        Box::new(ConnectionImpl{})
    }
}

pub trait Connection<'a> {

    fn create_statement<'b>(&'a mut self) -> Box<dyn Statement<'b>>;
}

struct ConnectionImpl {

}

impl<'a> Connection<'a> for ConnectionImpl {
    fn create_statement<'b>(&'a mut self) -> Box<dyn Statement<'b>> {
        Box::new(StatementImpl{})
    }
}

impl Drop for ConnectionImpl {
    fn drop(&mut self) {
        println!("drop");
    }
}

pub trait Statement<'a> {

}

struct StatementImpl {

}

impl<'a> Statement<'a> for StatementImpl {

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
