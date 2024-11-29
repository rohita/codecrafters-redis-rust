mod echo;
mod get;
mod ping;
mod set;
mod config;

use crate::resp::Value;
use anyhow::Result;
use crate::db;
use std::collections::HashMap;

use self::{
    get::Get, set::Set, ping::Ping, echo::Echo,
    config::Config,
};

pub trait Command {
    fn handle(
        &self, storage:
        &mut db::Db,
        config: HashMap::<String, String>) -> Value;
}

pub fn from(value: Value) -> Box<dyn Command> {
    let (command, args) = extract_command(value).unwrap();
    let return_value: Box<dyn Command> = {
        match command.to_lowercase().as_str() {
            "ping" => Box::new(Ping::new()),
            "echo" => Box::new(Echo::new(args)),
            "set" => Box::new(Set::new(args)),
            "get" => Box::new(Get::new(args)),
            "config" => Box::new(Config::new(args)),
            c => panic!("Cannot handle command {}", c),
        }
    };

    return_value
}

fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => {
            Ok((
                a.first().unwrap().clone().unpack_str(),
                a.into_iter().skip(1).collect(),
            ))
        },
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}