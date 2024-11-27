#![allow(unused_imports)]
use std::net::{TcpListener,TcpStream};
use std::default::Default;
use anyhow::Result;
use resp::Value;

mod resp;
mod db;
mod command;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // stream represents the incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { // 'mut' allows us to modify the stream object
                println!("accepted new connection");

                // spawns a new thread for each incoming connection
                // 'move' lets the closure take ownership of 'stream'
                let _ = std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut handler = resp::RespHandler::new(stream);
    let mut storage = db::Db::default();

    println!("Starting read loop");
    loop {
        let value = handler.read_value().unwrap();
        println!("Got value {:?}", value);

        let response = if let Some(v) = value {
            let comm = command::from(v);
            comm.handle(&mut storage)
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).unwrap();
    }
}