use std::net::TcpStream;

use crate::database::database_error::DatabaseError;

use super::column_type::ColumnType;

pub struct TypeStream {
    tcp_stream: TcpStream,
}

impl TypeStream {
    pub fn read_u8(&mut self) -> Result<u8, DatabaseError> {
        todo!()
    }

    pub fn read_type(&mut self) -> Result<ColumnType, DatabaseError> {
        todo!()
    }

    pub fn read_string(&mut self) -> Result<String, DatabaseError> {
        todo!()
    }

    pub fn new(tcp_stream: TcpStream) -> TypeStream {
        TypeStream { tcp_stream }
    }

    pub fn write_type(&mut self, value: &ColumnType) -> Result<(), DatabaseError> {
        todo!()
    }

    pub fn write_u8(&mut self, value: &u8) -> Result<(), DatabaseError> {
        todo!()
    }

    pub fn write_u64(&mut self, _value: &u64) -> Result<(), DatabaseError> {
        todo!()
    }

    pub fn write_string(&self, _user: &str) -> Result<(), DatabaseError> {
        todo!()
    }

    pub fn write(tcp_stream: &mut TcpStream, buf: &[u8]) -> Result<(), DatabaseError> {
        todo!()
    }
}
