use std::collections::VecDeque;

use crate::database::database_error::DatabaseError;

use super::{
    protocol_stream::{DriverProtocolCommand, DriverProtocolStream},
    row::Row,
};

pub const DEFAULT_FETCH_SIZE: u64 = 256;

struct Column {

}

pub struct ResultSet<'a> {
    stream: &'a mut DriverProtocolStream,
    fetch_size: u64,
    columns: Vec<Column>,
    rows: VecDeque<Row>,
}

impl<'a> ResultSet<'a> {
    pub fn new(stream: &mut DriverProtocolStream) -> ResultSet {
        ResultSet {
            stream,
            fetch_size: DEFAULT_FETCH_SIZE,
            columns: Vec::new(),
            rows: VecDeque::new(),
        }
    }

    fn read_metadata(&mut self) -> Result<(), DatabaseError> {
        // FIXME read number and type of columns
        match self.stream.read_command() {
            Ok(command) => {
                match command {
                    DriverProtocolCommand::U8{ value } => {
                       for _ in 1..value {
                            self.read_column()?
                        }
                        Ok(())
                    },
                    _ => Err(DatabaseError::ProtocolViolation)
                }
            }
            _ => Err(DatabaseError::ProtocolViolation)
        }
    }

    fn read_column(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }

    fn read_row(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }

    fn fetch(&mut self) -> Result<(), DatabaseError> {
        match self.stream.write_command(&DriverProtocolCommand::Fetch {
            fetch_size: self.fetch_size,
        }) {
            Ok(()) => {
                loop {
                    match self.stream.read_command() {
                        Ok(DriverProtocolCommand::Row) => { self.read_row()?; },
                        Ok(DriverProtocolCommand::Ready) => break Ok(()),
                        Ok(_) => break Err(DatabaseError::ProtocolViolation),
                        Err(database_error) => break Err(database_error),
                    }
                }
            },
            Err(database_error) => Err(database_error),
        }
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

    pub fn get_string(&self, column: &str) -> Result<String, DatabaseError> {
        if (self.rows.is_empty()) {
            Err(DatabaseError::IllegalState)
        } else {
            match self.rows.front() {
                Some(row) => row.get_string(column),
                None => Err(DatabaseError::Defect),
            }
        }
    }
}
