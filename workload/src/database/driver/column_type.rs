use crate::database::database_error::DatabaseError;

pub enum ColumnType {
    String,
    Number,
    Boolean,
    Date,
}

impl ColumnType {

    pub(crate) fn get_string(&self, value: &[u8]) -> Result<Option<String>, DatabaseError> {
        match (self) {
            ColumnType::String => todo!(),
            ColumnType::Number => todo!(),
            ColumnType::Boolean => todo!(),
            ColumnType::Date => todo!(),
        }
    }
}
