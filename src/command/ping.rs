use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Ping { }

impl Ping {
    pub fn new() -> Self {
        Ping {}
    }
}

impl Command for Ping {
    fn handle(&self, _storage: &mut db::Db, _config: HashMap::<String, String>) -> Value {
        Value::SimpleString("PONG".to_string())
    }
}