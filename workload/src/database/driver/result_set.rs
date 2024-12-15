use super::{
    column_type::ColumnType,
    command_stream::{Command, CommandStream},
};
use crate::database::database_error::DatabaseError;
use std::collections::{HashMap, VecDeque};

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

pub struct ResultSet<'a> {
    stream: &'a mut CommandStream,
    fetch_size: u64,
    index_for_name: HashMap<String, usize>,
    column_types: Vec<ColumnType>,
    columns: Vec<Column>,
    rows: VecDeque<Vec<Vec<u8>>>,
}

impl<'a> ResultSet<'a> {
    pub fn new(stream: &mut CommandStream, column_names: Vec<String>, column_types: Vec<ColumnType>) -> ResultSet {
        let mut index_for_name = HashMap::<String, usize>::new();
        let mut i= 0;
        column_names.into_iter().for_each(|column_name| {
            index_for_name.insert(column_name, i);
            i += 1
        });
        ResultSet {
            stream,
            fetch_size: DEFAULT_FETCH_SIZE,
            index_for_name,
            column_types,
            columns: Vec::new(),
            rows: VecDeque::new(),
        }
    }

    fn fetch(&mut self) -> Result<(), DatabaseError> {
        self.stream.write_command(&Command::Fetch {
            fetch_size: self.fetch_size,
        })?;
        loop {
            match self.stream.read_command() {
                Ok(Command::Row{ values }) => self.rows.push_back(values),
                Ok(Command::Ready) => break Ok(()),
                Ok(_) => break Err(DatabaseError::ProtocolViolation),
                Err(err) => break Err(err),
            }
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

    pub fn get_string(&self, column: &str) -> Result<Option<String>, DatabaseError> {
        if self.rows.is_empty() {
            Err(DatabaseError::IllegalState)
        } else {
            match self.rows.front() {
                Some(row) => {
                    match self.index_for_name.get(column) {
                        Some(index) => {
                            match row.get(*index) {
                                Some(value) => {
                                    match self.column_types.get(*index) {
                                        Some(column_type) => Ok(column_type.get_string(value)),
                                        None => Err(DatabaseError::Defect)
                                    }
                                },
                                None => Ok(None),
                            }
                        },
                        None => Err(DatabaseError::NoSuchColumn),
                    }
                },
                None => Err(DatabaseError::Defect),
            }
        }
    }
}
