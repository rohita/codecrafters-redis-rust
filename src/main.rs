#![allow(unused_imports)]
use std::net::{TcpListener,TcpStream};
use anyhow::Result;
use resp::Value;

mod resp;

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
    let mut storage: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    println!("Starting read loop");
    loop {
        let value = handler.read_value().unwrap();
        println!("Got value {:?}", value);

        let response = if let Some(v) = value {
            let (command, args) = extract_command(v).unwrap();
            match command.to_lowercase().as_str() {
                "ping" => Value::SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                "set" => set(&mut storage, unpack_bulk_str(args[0].clone()).unwrap(), unpack_bulk_str(args[1].clone()).unwrap()),
                "get" => get(&storage, unpack_bulk_str(args[0].clone()).unwrap()),
                c => panic!("Cannot handle command {}", c),
            }
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).unwrap();
    }
}

fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => {
            Ok((
                unpack_bulk_str(a.first().unwrap().clone())?,
                a.into_iter().skip(1).collect(),
            ))
        },
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}

fn unpack_bulk_str(value: Value) -> Result<String> {
    match value {
        Value::BulkString(s) => Ok(s),
        _ => Err(anyhow::anyhow!("Expected command to be a bulk string"))
    }
}

fn set(storage: &mut std::collections::HashMap<String, String>, key: String, value: String) -> Value {
    storage.insert(key, value);
    Value::SimpleString("OK".to_string())
}

fn get(storage: &std::collections::HashMap<String, String>, key: String) -> Value {
    match storage.get(&key) {
        Some(v) => Value::BulkString(v.to_string()),
        None => Value::Null,
    }
}