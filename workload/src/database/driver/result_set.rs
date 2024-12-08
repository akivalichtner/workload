use crate::database::database_error::DatabaseError;

use super::{protocol_stream::{DriverProtocolCommand, DriverProtocolStream}, row::Row};

pub const DEFAULT_FETCH_SIZE: u64 = 256;

pub struct ResultSet<'a> {

    driver_protocol_stream: &'a mut DriverProtocolStream,
    fetch_size: u64,
    rows: Vec<Row>,
}

impl<'a> ResultSet<'a> {

    pub fn new(driver_protocol_stream: &mut DriverProtocolStream) -> ResultSet {
        ResultSet {
            driver_protocol_stream,
            fetch_size: DEFAULT_FETCH_SIZE,
            rows: Vec::new(),
        }
    }

    fn fetch(&mut self) -> Result<(), DatabaseError> {
        todo!();
    }

    pub fn has_next(&mut self) -> bool {
        // if vector empty
        //   fetch
        // if vector not empty
        //   true
        // else 
        //   false
        self.fetch();
        if self.rows.is_empty() {
            self.fetch();
        }
        !self.rows.is_empty()
    }

    pub fn next(&self) {
        todo!()
    }

    pub fn get_string(&self, _column: &str) {
        todo!()
    }
}
