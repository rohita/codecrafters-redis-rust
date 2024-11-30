#![allow(dead_code)]
use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;
use crate::DEFAULT_MASTER_REPLID;
use crate::DEFAULT_MASTER_OFFSET;

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
        let role = match storage.config().get("replicaof") {
            Some(_v) => "slave",
            None => "master",
        };

        Value::BulkString(
        format!("role:{}\r\nmaster_replid:{}\r\nmaster_repl_offset:{}", role, DEFAULT_MASTER_REPLID, DEFAULT_MASTER_OFFSET)
        )
    }
}