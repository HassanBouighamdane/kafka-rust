#![allow(unused_imports)]
use std::{io::{BufReader, Write}, net::{TcpListener, TcpStream}};
mod response;
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

    let header=ResponseHeaderV0::new(7);
    let response=Response::new(0, header, String::new());
    if let Err(e)= stream.write_all(response.to_string().as_bytes()){
        println!("Error writing to stream: {}",e);
    }
}
