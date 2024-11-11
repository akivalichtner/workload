
pub fn new_data_source() -> Box<dyn DataSource> {
    Box::new(DataSourceImpl{})
}

pub trait DataSource {
    
}

struct DataSourceImpl {
}

impl DataSource for DataSourceImpl {

}

fn _test() {
    let _data_source_factory = new_data_source();

}