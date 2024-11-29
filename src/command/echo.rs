use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Echo {
    pub text: String,
}

impl Echo {
    pub fn new(args: Vec<Value>) -> Self {
        Echo {
            text: args[0].clone().unpack_str(),
        }
    }
}

impl Command for Echo {
    fn handle(&self, _storage: &mut db::Db, _config: HashMap::<String, String>) -> Value {
        Value::BulkString(self.text.clone())
    }
}