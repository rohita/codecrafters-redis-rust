use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: u128,
}

#[derive(Clone)]
pub struct Db {
    pub storage: HashMap<String, Item>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, expires: u128) {
        let item = Item {
            value,
            created: Instant::now(),
            expires,
        };
        self.storage.insert(key, item);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let item = self.storage.get(&key)?;
        let is_expired =
            item.expires > 0 && item.created.elapsed().as_millis() > item.expires;

        match is_expired {
            true => None,
            false => Some(item.value.clone()),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.storage.keys().cloned().collect()
    }
}

impl Default for Db {
    fn default() -> Self {
        Db::new()
    }
}