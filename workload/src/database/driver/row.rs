use crate::database::database_error::DatabaseError;


pub struct Row {

}

impl Row {

    pub fn get_string(&self, _column: &str) -> Result<String, DatabaseError> {
        todo!()
    }
}