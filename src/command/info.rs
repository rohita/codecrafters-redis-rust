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
    fn handle(&self, _storage: &mut db::Db) -> Value {
        Value::BulkString("role:master".to_string())
    }
}