use super::object::Object;
use std::collections::HashMap;

pub struct Db {
    dict: HashMap<String, Object>,
}
impl Db {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: String, value: String) {
        self.dict.insert(key, Object::STRING(value));
    }
}
