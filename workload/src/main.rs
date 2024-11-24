use driver::*;

mod driver;

fn main() -> Result<(), DatabaseError> {
    let data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
    let connection = data_source.get_connection()?;
    let statement = connection.create_statement();
    let row_count = statement.execute_update("INSERT INTO t (c) VALUES (1)")?;
    println!("row count: {}", row_count);
    let result_set = statement.execute_query("SELECT c FROM t")?;
    while result_set.has_next() {
        result_set.next();
        result_set.get_string("c");
    }
    connection.commit()?;
    Ok(())
}
