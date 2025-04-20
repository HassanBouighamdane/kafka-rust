#![allow(unused_imports)]
use std::{io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}};
mod response;
mod request;
use bytes::{buf, Buf};
use request::Request;
use response::{Response,ResponseHeaderV0};


fn main() {

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    println!("The server is listening at : http://127.0.0.1:9092");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_request(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream:TcpStream){
    let buf_reader=BufReader::new(&stream);
    let request=parse_request(buf_reader);

    let header=ResponseHeaderV0::new(request.correlation_id);
    let response=Response::new(0, header, String::new());
    if let Err(e)= stream.write_all(&response.to_bytes()){
        println!("Error writing to stream: {}",e);
    }
}

fn parse_request(mut buf_reader:BufReader<&TcpStream>)->Request{

    let mut buf =[0u8;4];
    buf_reader.read_exact(&mut buf).unwrap();
    let message_size = i32::from_be_bytes(buf);

    let mut buf = [0u8; 2];
    buf_reader.read_exact(&mut buf).unwrap();
    let request_api_key = i16::from_be_bytes(buf);

    let mut buf = [0u8; 2];
    buf_reader.read_exact(&mut buf).unwrap();
    let request_api_version = i16::from_be_bytes(buf);

    let mut buf = [0u8; 4];
    buf_reader.read_exact(&mut buf).unwrap();
    let correlation_id = i32::from_be_bytes(buf);
 
    let client_id=String::new();

    Request { 
        message_size,
        request_api_key,
         request_api_version, 
         correlation_id, 
         client_id }

}
