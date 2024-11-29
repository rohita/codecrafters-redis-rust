#![allow(unused_imports)]
use std::net::{TcpListener,TcpStream};
use std::default::Default;
use std::collections::HashMap;
use std::env;
use std::fs;
use anyhow::Result;
use resp::Value;

mod resp;
mod db;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let mut config = HashMap::<String, String>::new();
    if args.len() > 2 && (args[1] == "--dir" || args[3] == "--dbfilename") {
        config.insert("dir".to_string(), args[2].to_string());
        config.insert("dbfilename".to_string(), args[4].to_string());
    }
    println!("Config: {:?}", config);

    let storage = db::Db::from_config(config.clone());

    // stream represents the incoming connection
    for stream in listener.incoming() {
        let storage = storage.clone();

        match stream {
            Ok(stream) => { // 'mut' allows us to modify the stream object
                println!("accepted new connection");

                // spawns a new thread for each incoming connection
                // 'move' lets the closure take ownership of 'stream'
                let _ = std::thread::spawn(move || handle_client(stream, storage));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, mut storage: db::Db) {
    let mut handler = resp::RespHandler::new(stream);

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
