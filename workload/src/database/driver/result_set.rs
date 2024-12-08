use crate::database::database_error::DatabaseError;

use super::{protocol_stream::{DriverProtocolCommand, DriverProtocolStream}, row::Row};

pub const DEFAULT_FETCH_SIZE: u64 = 256;

pub struct ResultSet<'a> {

    driver_protocol_stream: &'a mut DriverProtocolStream,
    fetch_size: u64,
    rows: Vec<Row>,
    position: usize,
}

impl<'a> ResultSet<'a> {

    pub fn new(driver_protocol_stream: &mut DriverProtocolStream) -> ResultSet {
        ResultSet {
            driver_protocol_stream,
            fetch_size: DEFAULT_FETCH_SIZE,
            rows: Vec::new(),
            position: 0,
        }
    }

    fn fetch(&mut self) -> Result<(), DatabaseError> {
        todo!();
    }

    pub fn has_next(&mut self) -> bool {
        if self.rows.is_empty() {
            self.fetch();
        }
        !self.rows.is_empty()
    }

    pub fn next(&mut self) -> Result<(), DatabaseError> {
        self.position += 1;
        if self.position < self.rows.len() {
            Ok(())
        } else {
            Err(DatabaseError::IllegalState)
        }
    }

    pub fn get_string(&self, _column: &str) {
        todo!()
    }
}
