use std::net::{TcpListener,TcpStream};
use crate::resp::RespHandler;
use crate::Value;

pub struct ReplicaClient {
    handler: RespHandler,
}

impl ReplicaClient {
    pub fn connect(address: String) -> Self {
        let sock = TcpStream::connect(address).unwrap();
        let handler = RespHandler::new(sock);
        ReplicaClient { handler }
    }

    pub fn ping(&mut self) -> Value {
        let command = Value::Array(vec![Value::BulkString("PING".to_string())]);
        self.call(command)
    }

    pub fn replconf(&mut self, key: String, value: String) -> Value {
        let mut items = vec![];
        items.push(Value::BulkString("REPLCONF".to_string()));
        items.push(Value::BulkString(key));
        items.push(Value::BulkString(value));
        let command = Value::Array(items);
        self.call(command)
    }

    pub fn psync(&mut self) -> Value {
        let mut items = vec![];
        items.push(Value::BulkString("PSYNC".to_string()));
        items.push(Value::BulkString("?".to_string()));
        items.push(Value::BulkString("-1".to_string()));
        let command = Value::Array(items);
        self.call(command)
    }

    fn call(&mut self, cmd: Value) -> Value {
        self.handler.write_value(cmd).unwrap();
        let resp = self.handler.read_value().unwrap();
        if let Some(v) = resp {
            v
        } else {
            Value::Null
        }
    }
}