use honeycomb::Db;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    println!("Connection established");
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "OK";
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    let mut db = Db::new();
    {
        let version: &str = env!("CARGO_PKG_VERSION");
        db.add("__version__".to_string(), version.to_string());
    }
    let listener = TcpListener::bind("127.0.0.1:6636")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
