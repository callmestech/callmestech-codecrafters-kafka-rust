#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        let result = stream.and_then(|mut stream| {
            println!("accepted new connection");
            let message_id = 0i32.to_be_bytes();
            let corelation_id = 7i32.to_be_bytes();

            let response = [message_id, corelation_id].concat();
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
