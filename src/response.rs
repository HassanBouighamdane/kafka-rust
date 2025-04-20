use std::fmt::Display;

pub struct Response{
    message_size:i32,
    header:ResponseHeaderV0,
    body:String
}


pub struct ResponseHeaderV0(i32);

impl ResponseHeaderV0{
    pub fn new(correlation_id:i32)->ResponseHeaderV0{
        ResponseHeaderV0(correlation_id)
    }
}
impl Response{
    pub fn new(message_size:i32,header:ResponseHeaderV0,body:String)->Self{
        Self { message_size, header, body}
    }
}

impl Display for ResponseHeaderV0{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Display for Response{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}\r\n{}",self.message_size,self.header.0)
    }
}