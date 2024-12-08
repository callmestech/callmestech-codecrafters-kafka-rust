#[allow(unused_imports)]
use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use bytes::BytesMut;
use codecrafters_kafka::domain::Response;

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    // read request from the stream

    while stream.peek(&mut [0u8; 1]).is_ok() {
        println!("Accepting connection");
        let mut reader = BufReader::new(&stream);
        // read the first 4 bytes to get the size of the request
        let mut req_size = [0u8; 4];
        reader.read_exact(&mut req_size)?;
        let req_size = i32::from_be_bytes(req_size);
        println!("Reading request of {} bytes", req_size);
        // read req_size bytes to get the request
        let mut buf = vec![0u8; req_size as usize];
        reader.read_exact(&mut buf)?;
        println!("Received {} bytes", buf.len());
        // prepend the size of the request to the
        let buf = [req_size.to_be_bytes().to_vec(), buf].concat();

        let bytes_mut = BytesMut::from(&buf[..]);
        let request = bytes_mut.into();
        println!("Request: {:?}", request);
        let response = Response::from(&request);

        println!("Response: {:?}", response);
        println!("Response size: {}", response.message_size());

        let response_bytes: BytesMut = response.into();

        stream.write_all(&response_bytes)?;

    }
    Ok(())
}
