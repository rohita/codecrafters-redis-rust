use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Info {
    pub section: String,
}

impl Info {
    pub fn new(args: Vec<Value>) -> Self {
        Info {
            section: args[0].clone().unpack_str(),
        }
    }
}

impl Command for Info {
    fn handle(&self, storage: &mut db::Db) -> Value {
        match storage.config().get("replicaof") {
            Some(_v) => {
                Value::BulkString("role:slave".to_string())
            },
            None => Value::BulkString("role:master".to_string()),
        }
    }
}