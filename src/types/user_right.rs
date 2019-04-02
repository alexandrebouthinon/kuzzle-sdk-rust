#[derive(Deserialize, Clone, Debug)]
pub struct UserRight {
    #[serde(alias = "controller")]
    _controller: String,
    #[serde(alias = "action")]
    _action: String,
    #[serde(alias = "index")]
    _index: String,
    #[serde(alias = "collection")]
    _collection: String,
    #[serde(alias = "value")]
    _value: String,
}

impl UserRight {
    pub fn controller(&self) -> String {
        self._controller.clone()
    }

    pub fn action(&self) -> String {
        self._action.clone()
    }

    pub fn index(&self) -> String {
        self._index.clone()
    }

    pub fn collection(&self) -> String {
        self._collection.clone()
    }

    pub fn value(&self) -> String {
        self._value.clone()
    }
}
