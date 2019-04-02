use crate::controllers::*;
use crate::protocols::Protocol;
use crate::types::{KuzzleRequest, KuzzleResponse, QueryOptions};
use std::error::Error;

/// Kuzzle is the Kuzzle SDK client used to dial with the Kuzzle server.
pub struct Kuzzle {
    _protocol: Box<Protocol>,
    _jwt: Option<String>,
}

impl Kuzzle {
    /// Kuzzle SDK constructor
    ///
    /// # Arguments
    ///
    /// * `protocol` - A struct implementing the `protocols::Protocol` trait
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let _kuzzle = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
    /// ```
    pub fn new<P>(protocol: P) -> Kuzzle
    where
        P: 'static + Protocol,
    {
        Kuzzle {
            _protocol: Box::new(protocol),
            _jwt: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<Error>> {
        if self._protocol.is_ready() {
            return Ok(());
        }

        self._protocol.connect()
    }

    /// Execute the given KuzzleRequest and returns a `Result` which contains
    /// `KuzzleResponse` if execute was ok or a `KuzzleError` else.
    pub fn query(
        &self,
        req: KuzzleRequest,
        options: QueryOptions,
    ) -> Result<KuzzleResponse, Box<Error>> {
        let request = match &self._jwt {
            Some(jwt) => req.clone().set_jwt(jwt.to_string()),
            None => req.clone(),
        };

        self._protocol.send(request, options)

    }

    /// Kuzzle JWT getter
    pub fn jwt(&self) -> String {
        self._jwt.clone().unwrap_or(String::new())
    }

    /// Kuzzle JWT setter
    pub fn set_jwt(&mut self, jwt: String) {
        self._jwt = Some(jwt);
    }

    /// Kuzzle AuthController's getter
    pub fn auth(&mut self) -> AuthController {
        AuthController(self)
    }

    /// Kuzzle BulkController's getter
    pub fn bulk(&mut self) -> BulkController {
        BulkController(self)
    }

    /// Kuzzle CollectionController's getter
    pub fn collection(&mut self) -> CollectionController {
        CollectionController(self)
    }

    /// Kuzzle DocumentController's getter
    pub fn document(&mut self) -> DocumentController {
        DocumentController(self)
    }

    /// Kuzzle IndexController's getter
    pub fn index(&mut self) -> IndexController {
        IndexController(self)
    }

    /// Kuzzle MemoryStorageController's getter
    pub fn ms(&mut self) -> MemoryStorageController {
        MemoryStorageController(self)
    }

    /// Kuzzle RealtimeController's getter
    pub fn realtime(&mut self) -> RealtimeController {
        RealtimeController(self)
    }

    /// Kuzzle SecurityController's getter
    pub fn security(&mut self) -> SecurityController {
        SecurityController(self)
    }

    /// Kuzzle ServerController's getter
    pub fn server(&mut self) -> ServerController {
        ServerController(self)
    }
}
