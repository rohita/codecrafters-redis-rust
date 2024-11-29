use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Keys {
    pub pattern: String,
}

impl Keys {
    pub fn new(args: Vec<Value>) -> Self {
        Keys {
            pattern: args[0].clone().unpack_str(),
        }
    }
}

impl Command for Keys {
    fn handle(&self, storage: &mut db::Db) -> Value {
        let keys = storage
            .keys()
            .iter()
            .map(|s| Value::BulkString(s.to_string()))
            .collect::<Vec<_>>();
        Value::Array(keys)
    }
}