use crate::command::Command;
use crate::Value;
use crate::db;
use std::collections::HashMap;

pub struct Set {
    pub key: String,
    pub value: String,
    pub options: Vec<Value>,
}

impl Set {
    pub fn new(args: Vec<Value>) -> Self {
        Set {
            key: args[0].clone().unpack_str(),
            value: args[1].clone().unpack_str(),
            options: args.into_iter().skip(2).collect::<Vec<Value>>(),
        }
    }

    pub fn expires(&self) -> u128 {
        if let Some(Value::BulkString(s)) = self.options.first() {
            match s.to_lowercase().as_str() {
                "px" => {
                    self.options[1].clone().unpack_str().parse::<u128>().unwrap()
                },
                o => panic!("Unknown option: {:?}", o),
            }
        } else {
            0
        }
    }
}

impl Command for Set {
    fn handle(&self, storage: &mut db::Db) -> Value {
        println!("Setting key: {}, value: {}, expires: {}", self.key, self.value, self.expires());
        storage.set(self.key.clone(), self.value.clone(), self.expires());
        Value::SimpleString("OK".to_string())
    }
}