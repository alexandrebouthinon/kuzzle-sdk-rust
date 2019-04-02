#[derive(Deserialize)]
pub struct TokenValidity {
    #[serde(alias = "valid")]
    _valid: bool,
    #[serde(alias = "state")]
    _state: Option<String>,
    #[serde(alias = "expiresAt")]
    _expires_at: Option<i64>,
}

impl TokenValidity {
    pub fn valid(&self) -> bool {
        self._valid
    }

    pub fn state(&self) -> String {
        match &self._state {
            Some(state) => state.clone(),
            None => String::new(),
        }
    }

    pub fn expires_at(&self) -> i64 {
        match self._expires_at {
            Some(timestamp) => timestamp,
            None => 0,
        }
    }
}
