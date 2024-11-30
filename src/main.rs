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
    let mut config = HashMap::<String, String>::new();
    println!("{:?}", args);

    for index in (1..args.len()).step_by(2) {
        let config_key = args[index].get(2..).unwrap().to_string();
        config.insert(config_key, args[index + 1].to_string());
    }

    println!("Config: {:?}", config);
    let port: String = config.get("port").unwrap_or(&"6379".to_string()).to_string();
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
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
