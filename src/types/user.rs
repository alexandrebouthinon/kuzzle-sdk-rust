use std::collections::HashMap;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(alias = "_id")]
    _id: String,
    #[serde(alias = "_source")]
    _content: HashMap<String, Value>,
    #[serde(alias = "strategies")]
    _strategies: Vec<String>,
}

impl User {
    pub fn id(&self) -> String {
        self._id.clone()
    }

    pub fn content(&self) -> HashMap<String, Value> {
        self._content.clone()
    }

    pub fn strategies(&self) -> Vec<String> {
        self._strategies.clone()
    }
}
