#[allow(unused_imports)]
use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use bytes::{Buf, BufMut};
use codecrafters_kafka::domain::Error;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut message_size = [0; 4];
    stream.read_exact(&mut message_size).unwrap();
    let message_size = i32::from_be_bytes(message_size) as usize;

    let mut request = vec![0; message_size];
    stream.read_exact(&mut request).unwrap();
    let mut request = request.as_slice();

    let _request_api_key = request.get_i16();
    let _request_api_version = request.get_i16();
    let corelation_id = request.get_i32();
    let error_code = Error::UnsupportedVersion.error_code();

    let mut response = [0i32.to_be_bytes(), corelation_id.to_be_bytes()].concat();
    response.put_u16(error_code);
    println!("Response: {:?}", response);
    println!("Response size: {}", response.len());

    stream.write_all(&response).unwrap();
}
