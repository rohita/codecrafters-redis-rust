use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;
use crate::DEFAULT_MASTER_REPLID;
use crate::DEFAULT_MASTER_OFFSET;

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
        Value::SimpleString(
            format!("FULLRESYNC {} {}", DEFAULT_MASTER_REPLID, DEFAULT_MASTER_OFFSET)
        )
    }
}