use driver::*;

mod driver;

fn main() {
    let data_source = DataSource::new("myname", 8080, "myuser", "mypassword");
    match data_source.get_connection() {
        Ok(connection) => {
            let statement = connection.create_statement();
            match statement.execute_update("INSERT INTO t (c) VALUES (1)") {
                Ok(row_count) => {
                    println!("Updated {} rows", row_count);
                }
                Err(database_error) => {
                    println!("Update failed: {}", database_error);
                }
            }
            match statement.execute_query("SELECT c FROM t") {
                Ok(result_set) => {
                    while result_set.has_next() {
                        result_set.next();
                        result_set.get_string("c");
                    }
                    connection.commit();
                }
                Err(database_error) => {
                    println!("Query failed: {}", database_error);
                }
            }
        }
        Err(error) => {
            println!("Error connection to database: {}", error);
        }
    }
}
