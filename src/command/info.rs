#![allow(dead_code)]
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
        let role = match storage.config().get("replicaof") {
            Some(_v) => "slave",
            None => "master",
        };

        Value::BulkString(
        format!("role:{}\r\nmaster_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb\r\nmaster_repl_offset:0", role)
        )
    }
}