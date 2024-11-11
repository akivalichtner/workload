
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
}

struct ConnectionImpl {

}

impl<'a> Connection<'a> for ConnectionImpl {

}

impl Drop for ConnectionImpl {
    fn drop(&mut self) {
        println!("drop");
    }
}

pub fn _test() {
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