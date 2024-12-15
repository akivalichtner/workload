use super::{
    column_type::ColumnType,
    command_stream::{DriverProtocolCommand, CommandStream},
};
use crate::database::database_error::DatabaseError;
use std::collections::VecDeque;

pub const DEFAULT_FETCH_SIZE: u64 = 256;

struct Column {
    name: String,
    column_type: ColumnType,
}

impl Column {
    fn new(name: String, column_type: ColumnType) -> Column {
        Column { name, column_type }
    }
}

struct Row {}

impl Row {
    pub fn get_string(&self, _column: &str) -> Result<String, DatabaseError> {
        todo!()
    }
}

pub struct ResultSet<'a> {
    stream: &'a mut CommandStream,
    fetch_size: u64,
    column_types: Vec<ColumnType>,
    columns: Vec<Column>,
    rows: VecDeque<Row>,
}

impl<'a> ResultSet<'a> {
    pub fn new(stream: &mut CommandStream, column_types: Vec<ColumnType>) -> ResultSet {
        ResultSet {
            stream,
            fetch_size: DEFAULT_FETCH_SIZE,
            column_types,
            columns: Vec::new(),
            rows: VecDeque::new(),
        }
    }

    pub fn read_metadata(&mut self) -> Result<(), DatabaseError> {
        // FIXME read number and type of columns
        match self.stream.read_command() {
            Ok(command) => match command {
                DriverProtocolCommand::U8 { value } => {
                    for _ in 1..value {
                        self.read_column_metadata()?
                    }
                    Ok(())
                }
                _ => Err(DatabaseError::ProtocolViolation),
            },
            _ => Err(DatabaseError::ProtocolViolation),
        }
    }

    fn read_column_metadata(&mut self) -> Result<(), DatabaseError> {
        let column_result = self.stream.read_command();
        let type_result = self.stream.read_command();
        match (column_result, type_result) {
            (
                Ok(DriverProtocolCommand::String { value: name }),
                Ok(DriverProtocolCommand::Type { value: column_type }),
            ) => {
                self.columns.push(Column::new(name, column_type));
                Ok(())
            }
            _ => Err(DatabaseError::ProtocolViolation),
        }
    }

    fn read_row(&mut self) -> Result<(), DatabaseError> {
        for column in &self.columns {
            match column.column_type {
                _ => {}
            }
        }
        Ok(())
    }

    fn fetch(&mut self) -> Result<(), DatabaseError> {
        match self.stream.write_command(&DriverProtocolCommand::Fetch {
            fetch_size: self.fetch_size,
        }) {
            Ok(()) => loop {
                match self.stream.read_command() {
                    Ok(DriverProtocolCommand::Row) => {
                        self.read_row()?;
                    }
                    Ok(DriverProtocolCommand::Ready) => break Ok(()),
                    Ok(_) => break Err(DatabaseError::ProtocolViolation),
                    Err(database_error) => break Err(database_error),
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
