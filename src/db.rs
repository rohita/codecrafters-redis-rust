use std::collections::HashMap;
use std::time::Instant;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Item {
    pub value: String,
    pub created: Instant,
    pub expires: u128,
}

#[derive(Clone)]
pub struct Db {
    config: HashMap::<String, String>,
    store: HashMap<String, Item>,
}

impl Db {
    pub fn from_config(config: HashMap::<String, String>) -> Self {
        let storage = read_file(config.clone());
        Db { config, store: storage }
    }

    pub fn set(&mut self, key: String, value: String, expires: u128) {
        let item = Item {
            value,
            created: Instant::now(),
            expires,
        };
        self.store.insert(key, item);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let item = self.store.get(&key)?;
        let is_expired =
            item.expires > 0 && item.created.elapsed().as_millis() > item.expires;

        match is_expired {
            true => None,
            false => Some(item.value.clone()),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.store.keys().cloned().collect()
    }

    pub fn config(&self) -> HashMap<String, String> {
        self.config.clone()
    }
}

fn read_file(config: HashMap<String, String>) -> HashMap<String, Item> {
    let mut storage = HashMap::new();

    if let Some(Ok(contents)) = config
        .get("dir")
        .map(|dir| {
            config
                .get("dbfilename")
                .map(|filename| format!("{dir}/{filename}"))
        })
        .flatten()
        .map(fs::read)
    {
        let mut iter = contents.into_iter().skip_while(|&b| b != 0xFB).skip(1);
        let hashtable_size = iter.next().unwrap() as usize;
        let expire_hashtable_size = iter.next().unwrap() as usize;
        println!("Hashtable Size: {}, Expire Size: {}", hashtable_size, expire_hashtable_size);

        for _ in 0..hashtable_size {
            let _value_type = iter.next();

            let key_len = iter.next().unwrap();
            let mut key_chars = Vec::new();
            for _ in 0..key_len {
                key_chars.push(iter.next().unwrap() as char);
            }
            let key = key_chars.into_iter().collect();

            let value_len = iter.next().unwrap();
            let mut value_chars = Vec::new();
            for _ in 0..value_len {
                value_chars.push(iter.next().unwrap() as char);
            }
            let value = value_chars.into_iter().collect();

            println!("Loaded from file = key: {:?}, value: {:?}", key, value);
            storage.insert(key, Item {
                value,
                created: Instant::now(),
                expires: 0,
            });
        }
    }

    storage
}

