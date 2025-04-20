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
    pub fn to_bytes(&self)->Vec<u8>{
        self.0.to_be_bytes().to_vec()
    }
}
impl Response{
    pub fn new(message_size:i32,header:ResponseHeaderV0,body:String)->Self{
        Self { message_size, header, body}
    }
    pub fn to_bytes(&self)->Vec<u8>{
        let mut bytes=Vec::new();

        bytes.extend_from_slice(&self.message_size.to_be_bytes());
        bytes.extend_from_slice(&self.header.0.to_be_bytes());

        bytes
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