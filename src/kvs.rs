use sled;

pub struct Kvs {
    db: sled::Db,
}

impl Kvs {
    pub fn new(file_path: &str) -> Result<Kvs, String> {
        let db = sled::open(file_path).map_err(|e| e.to_string())?;
        Ok(Kvs { db })
    }

    pub fn insert(&self, key: &str, value: &str) {
        self.db.insert(key, value.as_bytes()).unwrap();
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.db
            .get(key)
            .unwrap()
            .map(|v| std::str::from_utf8(&v).unwrap().to_string())
    }

    pub fn get_key_vec(&self) -> Vec<String> {
        self.db
            .iter()
            .keys()
            .map(|k| k.unwrap().to_vec())
            .map(|k| String::from_utf8(k).unwrap())
            .collect()
    }

    pub fn delete(&self, key: String) {
        self.db.remove(key).unwrap();
    }
}

impl Default for Kvs {
    fn default() -> Self {
        Self::new("/tmp/passmr/kvs").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let file_path = "/tmp/passmr/test_insert";
        let kvs = Kvs::new(file_path).unwrap();
        let key = "key";
        let value = "value";
        kvs.insert(key, value);
        assert_eq!(kvs.get("key"), Some("value".to_string()));
    }
}
