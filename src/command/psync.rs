use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Psync {

}

impl Psync {
    pub fn new(_args: Vec<Value>) -> Self {
        Psync {

        }
    }
}

impl Command for Psync {
    fn handle(&self, _storage: &mut db::Db) -> Value {
        Value::SimpleString("OK".to_string())
    }
}