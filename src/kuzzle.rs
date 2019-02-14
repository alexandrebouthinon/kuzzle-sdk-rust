use crate::controllers::*;
use crate::protocols::Protocol;
use crate::types::{KuzzleRequest, KuzzleResponse, QueryOptions};
use std::error::Error;

/// Kuzzle is the Kuzzle SDK client used to dial with the Kuzzle server.
pub struct Kuzzle {
    _protocol: Box<Protocol>,
    _jwt: String,
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
            _jwt: String::new(),
        }
    }

    /// Execute the given KuzzleRequest and returns a `Result` which contains
    /// `KuzzleResponse` if execute was ok or a `KuzzleError` else.
    pub fn query(
        &self,
        req: KuzzleRequest,
        options: QueryOptions,
    ) -> Result<KuzzleResponse, Box<Error>> {
        self._protocol.send(req, options)
    }

    /// Kuzzle JWT getter
    pub fn jwt(&self) -> String {
        self._jwt.clone()
    }

    /// Kuzzle JWT setter
    pub fn set_jwt(&mut self, jwt: String) {
        self._jwt = jwt;
    }

    /// Kuzzle AuthController's getter
    pub fn auth(&self) -> AuthController {
        AuthController(&self)
    }

    /// Kuzzle BulkController's getter
    pub fn bulk(&self) -> BulkController {
        BulkController(&self)
    }

    /// Kuzzle CollectionController's getter
    pub fn collection(&self) -> CollectionController {
        CollectionController(&self)
    }

    /// Kuzzle DocumentController's getter
    pub fn document(&self) -> DocumentController {
        DocumentController(&self)
    }

    /// Kuzzle IndexController's getter
    pub fn index(&self) -> IndexController {
        IndexController(&self)
    }

    /// Kuzzle MemoryStorageController's getter
    pub fn ms(&self) -> MemoryStorageController {
        MemoryStorageController(&self)
    }

    /// Kuzzle RealtimeController's getter
    pub fn realtime(&self) -> RealtimeController {
        RealtimeController(&self)
    }

    /// Kuzzle SecurityController's getter
    pub fn security(&self) -> SecurityController {
        SecurityController(&self)
    }

    /// Kuzzle ServerController's getter
    pub fn server(&self) -> ServerController {
        ServerController(&self)
    }
}
