#![allow(unused_imports)]
use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

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
    let mut buf_reader = BufReader::new(&stream);
    let mut buf: Vec<u8> = Vec::new();

    if let Ok(bytes_read) = buf_reader.read_to_end(&mut buf) {
        if bytes_read >= 12 {
            let message_id = 0i32.to_be_bytes();
            let corelation_id = &buf[8..12];
            let response = [&message_id, corelation_id].concat();

            if let Err(e) = stream.write_all(&response) {
                eprintln!("Error writing to stream: {}", e);
            }
        }
    } else {
        eprintln!("Error reading from stream");
    }
}
