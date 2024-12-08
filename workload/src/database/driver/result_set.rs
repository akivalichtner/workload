use super::protocol_stream::DriverProtocolStream;

pub struct ResultSet<'a> {
    driver_protocol_stream: &'a mut DriverProtocolStream,
}

impl<'a> ResultSet<'a> {
    pub fn new(driver_protocol_stream: &mut DriverProtocolStream) -> ResultSet {
        ResultSet {
            driver_protocol_stream,
        }
    }

    pub fn has_next(&self) -> bool {
        todo!()
    }

    pub fn next(&self) {
        todo!()
    }

    pub fn get_string(&self, _column: &str) {
        todo!()
    }
}
