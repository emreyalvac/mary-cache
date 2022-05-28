use hashbrown::HashMap;

pub struct Store {
    _cache: HashMap<String, String>,
}

// TODO: LRU Algorithm

impl Store {
    pub fn new() -> Self {
        Self {
            _cache: HashMap::default()
        }
    }

    pub fn get(&self, key: String) -> Result<String, bool> {
        let value = self._cache.get(key.as_str());
        match value {
            Some(value) => Ok(value.to_string()),
            None => Err(false)
        }
    }

    // TODO: Expiration time

    pub fn set(&mut self, key: String, value: String) -> Result<bool, bool> {
        self._cache.insert(key, value);
        Ok(true)
    }

    pub fn delete(&mut self, key: String) -> Result<String, bool> {
        let op = self._cache.remove(key.as_str());
        match op {
            Some(value) => Ok(value),
            None => Err(false)
        }
    }
}