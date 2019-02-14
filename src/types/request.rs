use serde_json::Value;
use std::collections::HashMap;

pub struct KuzzleRequest {
    _controller: String,
    _action: String,
    _index: Option<String>,
    _collection: Option<String>,
    _body: HashMap<String, Value>,
}

impl KuzzleRequest {
    pub fn new(controller: &str, action: &str) -> KuzzleRequest {
        KuzzleRequest {
            _controller: controller.to_string(),
            _action: action.to_string(),
            _index: None,
            _collection: None,
            _body: HashMap::new(),
        }
    }

    pub fn controller(&self) -> &String {
        &self._controller
    }

    pub fn action(&self) -> &String {
        &self._action
    }

    pub fn index(&self) -> &Option<String> {
        &self._index
    }

    pub fn collection(&self) -> &Option<String> {
        &self._collection
    }

    pub fn body(&self) -> &HashMap<String, Value> {
        &self._body
    }

    pub fn set_index(mut self, index: &str) -> Self {
        self._index = Some(index.to_string());
        self
    }

    pub fn add_to_body(mut self, key: String, value: Value) -> Self {
        self._body.insert(key, value);
        self
    }
}
