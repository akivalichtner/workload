pub enum ColumnType {
    String,
    Number,
    Boolean,
    Date,
}

impl ColumnType {

    pub(crate) fn get_string(&self, value: &[u8]) -> Option<String> {
        match (self) {
            ColumnType::String => todo!(),
            ColumnType::Number => todo!(),
            ColumnType::Boolean => todo!(),
            ColumnType::Date => todo!(),
        }
    }
}
