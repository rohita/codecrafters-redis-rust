use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Get {
    pub key: String,
}

impl Get {
    pub fn new(args: Vec<Value>) -> Self {
        Get {
            key: args[0].clone().unpack_str()
        }
    }
}

impl Command for Get {
    fn handle(&self, storage: &mut db::Db, _config: HashMap::<String, String>) -> Value {
        match storage.get(self.key.clone()) {
            Some(v) => Value::BulkString(v.to_string()),
            None => Value::Null,
        }
    }
}