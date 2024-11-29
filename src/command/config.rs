use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Config {
    pub subcommand: String,
    pub parameters: Vec<Value>,
}

impl Config {
    pub fn new(args: Vec<Value>) -> Self {
        Config {
            subcommand: args[0].clone().unpack_str(),
            parameters: args[1..].to_vec(),
        }
    }
}

impl Command for Config {
    fn handle(&self, _storage: &mut db::Db, config: HashMap::<String, String>) -> Value {
        match self.subcommand.as_str() {
            "GET" => {
                match config.get(&self.parameters[0].clone().unpack_str()) {
                    Some(v) => {
                        let mut items = vec![];
                        items.push(self.parameters[0].clone());
                        items.push(Value::BulkString(v.to_string()));
                        Value::Array(items)
                    },
                    None => Value::Null,
                }
            },
            _ => Value::Null
        }
    }
}