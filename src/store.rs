use hashbrown::HashMap;

pub struct Store {
    _cache: HashMap<String, prost_types::Any>,
}

// TODO: LRU Algorithm

impl Store {
    pub fn new() -> Self {
        Self {
            _cache: HashMap::default()
        }
    }

    pub fn get(&self, key: String) -> Result<prost_types::Any, bool> {
        let value = self._cache.get(key.as_str());
        match value {
            Some(value) => Ok(value.clone()),
            None => Err(false)
        }
    }

    // TODO: Expiration time

    pub fn set(&mut self, key: String, value: prost_types::Any) -> Result<bool, bool> {
        self._cache.insert(key, value);
        Ok(true)
    }

    pub fn delete(&mut self, key: String) -> Result<prost_types::Any, bool> {
        let op = self._cache.remove(key.as_str());
        match op {
            Some(value) => Ok(value),
            None => Err(false)
        }
    }

    pub fn cache_length(&self) -> i32 {
        self._cache.len() as i32
    }
}