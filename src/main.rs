#[allow(unused_imports)]
use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use bytes::BytesMut;
use codecrafters_kafka::domain::{Request, Response};

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
    // read request from the stream
    let mut reader = BufReader::new(&stream);
    let mut buf = vec![];
    reader.read_to_end(&mut buf).unwrap();

    let mut bytes_mut = BytesMut::from(&buf[..]);
    let request = Request::from(&mut bytes_mut);
    let response = Response::from(&request);

    println!("Response: {:?}", response);
    println!("Response size: {}", response.message_size());

    let response_bytes: BytesMut = response.into();

    stream.write_all(&response_bytes).unwrap();
}
