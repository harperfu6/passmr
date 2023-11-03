use sled;

struct Kvs {
    db: sled::Db,
}

impl Kvs {
    pub fn new(file_path: String) -> Result<Kvs, String> {
        let db = sled::open(file_path).map_err(|e| e.to_string())?;
        Ok(Kvs { db })
    }

    pub fn insert(&self, key: String, value: String) {
        self.db.insert(key, value.as_bytes()).unwrap();
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.db
            .get(key)
            .unwrap()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
    }

    pub fn delete(&self, key: String) {
        self.db.remove(key).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let file_path = "/tmp/passmr/test_insert";
        let kvs = Kvs::new(file_path.to_string()).unwrap();
        let key = "key";
        let value = "value";
        kvs.insert(key.to_string(), value.to_string());
        assert_eq!(kvs.get("key".to_string()), Some("value".to_string()));
    }
}
