
pub fn new_data_source() -> Box<dyn DataSource> {
    Box::new(DataSourceImpl{})
}

pub trait DataSource {

    fn get_connection(&mut self) -> Box<dyn Connection>;
}

pub trait Connection {
}

struct DataSourceImpl {
}

impl DataSource for DataSourceImpl {
    fn get_connection(&mut self) -> Box<dyn Connection> {
        todo!()
    }
}


fn _test() {
    let mut data_source = new_data_source();
    let _connection = data_source.get_connection();

}