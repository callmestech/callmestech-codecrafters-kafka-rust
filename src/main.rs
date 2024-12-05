#![allow(unused_imports)]
use std::{
    io::{BufReader, Read, Write},
    net::TcpListener,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        let result = stream.and_then(|mut stream| {
            println!("accepted new connection");
            let mut buf_reader = BufReader::new(&stream);
            let mut buf: Vec<u8> = Vec::new();
            let _request_size = buf_reader.read_to_end(&mut buf).unwrap();
            let corelation_id = &buf[8..12];
            let message_id = 0i32.to_be_bytes().to_vec();

            let response = [&message_id, corelation_id].concat();

            stream.write_all(&response)
        });
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
