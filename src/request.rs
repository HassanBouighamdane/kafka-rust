 pub struct Request{
    pub request_api_key:i16,
    pub request_api_version:i16,
    pub correlation_id:i32,
    pub client_id:String,
    //_tag_fields:Vec<T>
}

impl Request{
    pub fn new(  request_api_key:i16,
        request_api_version:i16,
        correlation_id:i32,
        client_id:String)->Self{
            Self { request_api_key,
                 request_api_version,
                  correlation_id,
                   client_id }
        } 
}