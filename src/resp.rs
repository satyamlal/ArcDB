pub enum RespValue{
    SimpleString(String),
}

impl RespValue {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            RespValue::SimpleString(s) => {
                let response = format!("+{}\r\n", s); // format: +<content>\r\n

                response.into_bytes()
            }
        }
    }
}