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

    let storage = read_file(config.clone());

    // stream represents the incoming connection
    for stream in listener.incoming() {
        let config = config.clone();
        let storage = storage.clone();

        match stream {
            Ok(stream) => { // 'mut' allows us to modify the stream object
                println!("accepted new connection");

                // spawns a new thread for each incoming connection
                // 'move' lets the closure take ownership of 'stream'
                let _ = std::thread::spawn(move || handle_client(stream, storage, config));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, mut storage: db::Db, config: HashMap<String, String>) {
    let mut handler = resp::RespHandler::new(stream);

    println!("Starting read loop");
    loop {
        let value = handler.read_value().unwrap();
        println!("Got value {:?}", value);

        let response = if let Some(v) = value {
            let comm = command::from(v);
            comm.handle(&mut storage, config.clone())
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).unwrap();
    }
}

fn read_file(config: HashMap<String, String>) -> db::Db {
    let mut storage = db::Db::default();

    if let Some(Ok(contents)) = config
        .get("dir")
        .map(|dir| {
            config
                .get("dbfilename")
                .map(|filename| format!("{dir}/{filename}"))
        })
        .flatten()
        .map(fs::read)
    {
        let mut iter = contents.into_iter().skip_while(|&b| b != 0xFB).skip(1);
        let _hashtable_size = iter.next();
        let _expire_hashtable_size = iter.next();
        let _value_type = iter.next();
        let key_len = iter.next().unwrap();
        let mut key_chars = Vec::new();
        for _ in 0..key_len {
            key_chars.push(iter.next().unwrap() as char);
        }
        let value_len = iter.next().unwrap();
        let mut value_chars = Vec::new();
        for _ in 0..value_len {
            value_chars.push(iter.next().unwrap() as char);
        }
        let key = key_chars.into_iter().collect();
        let value = value_chars.into_iter().collect();
        println!("Loaded from file = key: {:?}, value: {:?}", key, value);
        storage.set(key, value, 0);
    }

    storage
}