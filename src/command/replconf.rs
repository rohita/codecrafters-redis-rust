use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct ReplConf {

}

impl ReplConf {
    pub fn new(_args: Vec<Value>) -> ReplConf {
        ReplConf {}
    }
}

impl Command for ReplConf {
    fn handle(&self, _storage: &mut db::Db) -> Value {
        Value::SimpleString("OK".to_string())
    }
}