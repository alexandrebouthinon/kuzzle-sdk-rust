use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct KuzzleRequest {
    _controller: String,
    _action: String,
    _index: Option<String>,
    _collection: Option<String>,
    _strategy: Option<String>,
    _jwt: Option<String>,
    _body: HashMap<String, Value>,
    _query_strings: HashMap<String, Value>,
}

impl KuzzleRequest {
    pub fn new(controller: &str, action: &str) -> KuzzleRequest {
        KuzzleRequest {
            _controller: controller.to_string(),
            _action: action.to_string(),
            _index: None,
            _collection: None,
            _strategy: None,
            _jwt: None,
            _body: HashMap::new(),
            _query_strings: HashMap::new(),
        }
    }

    pub fn controller(&self) -> &String {
        &self._controller
    }

    pub fn action(&self) -> &String {
        &self._action
    }

    pub fn index(&self) -> String {
        match &self._index {
            Some(index) => index.clone(),
            None => String::new(),
        }
    }

    pub fn collection(&self) -> String {
        match &self._collection {
            Some(collection) => collection.clone(),
            None => String::new(),
        }
    }

    pub fn strategy(&self) -> String {
        match &self._strategy {
            Some(strategy) => strategy.clone(),
            None => String::new(),
        }
    }

    pub fn jwt(&self) -> String {
        match &self._jwt {
            Some(jwt) => jwt.clone(),
            None => String::new(),
        }
    }

    pub fn body(&self) -> &HashMap<String, Value> {
        &self._body
    }

    pub fn query_strings(&self) -> &HashMap<String, Value> {
        &self._query_strings
    }

    pub fn set_index(mut self, index: &str) -> Self {
        self._index = Some(index.to_string());
        self
    }

    pub fn set_strategy(mut self, strategy: &str) -> Self {
        self._strategy = Some(strategy.to_string());
        self
    }

    pub fn set_jwt(mut self, jwt: String) -> Self {
        self._jwt = Some(jwt);
        self
    }

    pub fn set_body(mut self, body: HashMap<String, Value>) -> Self {
        self._body = body;
        self
    }

    pub fn add_to_body(mut self, key: &str, value: Value) -> Self {
        self._body.insert(key.to_string(), value);
        self
    }

    pub fn add_to_query_strings(mut self, key: &str, value: Value) -> Self {
        self._query_strings.insert(key.to_string(), value);
        self
    }
}
