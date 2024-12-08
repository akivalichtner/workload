use std::collections::VecDeque;

use crate::database::database_error::DatabaseError;

use super::{protocol_stream::{DriverProtocolCommand, DriverProtocolStream}, row::Row};

pub const DEFAULT_FETCH_SIZE: u64 = 256;

pub struct ResultSet<'a> {

    driver_protocol_stream: &'a mut DriverProtocolStream,
    fetch_size: u64,
    rows: VecDeque<Row>,
}

impl<'a> ResultSet<'a> {

    pub fn new(driver_protocol_stream: &mut DriverProtocolStream) -> ResultSet {
        ResultSet {
            driver_protocol_stream,
            fetch_size: DEFAULT_FETCH_SIZE,
            rows: VecDeque::new(),
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
        if self.rows.is_empty() {
            Err(DatabaseError::IllegalState)
        } else {
            self.rows.pop_front();
            Ok(())
        }
    }

    pub fn get_string(&self, _column: &str) {
        todo!()
    }
}
