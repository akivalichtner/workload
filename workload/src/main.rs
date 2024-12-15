use crate::database::database_error::DatabaseError;
use crate::database::driver::data_source::*;

mod database;

fn main() -> Result<(), DatabaseError> {
    let data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
    let mut connection = data_source.get_connection()?;
    let mut statement = connection.create_statement()?;
    let row_count = statement.execute_update("INSERT INTO t (c) VALUES (1)")?;
    println!("row count: {}", row_count);
    let mut result_set = statement.execute_query("SELECT c FROM t")?;
    while result_set.has_next() {
        result_set.next()?;
        result_set.get_string("c")?;
    }
    connection.commit()?;
    Ok(())
}
