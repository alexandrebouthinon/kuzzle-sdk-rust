use std::collections::HashMap;

type Routes = HashMap<String, HashMap<String, Route>>;

#[derive(Deserialize, Clone)]
pub struct Route {
    pub url: String,
    pub verb: String,
}

use crate::protocols::State;
use crate::types::KuzzleOptions;

pub struct Http {
    _client: Client,
    _options: KuzzleOptions,
    _routes: Routes,
    _state: State,
}

use std::fs::File;
use std::io::Read;

impl Http {
    /// Returns a Http struct that acts as an HTTP
    /// client to dial with Kuzzle server.
    /// Perhaps, Kuzzle HTTP routes are loaded from a JSON file.
    ///
    /// # Arguments
    /// * `options` - An `types::Options` used to configure Http dialer
    ///
    /// # Example
    /// ```
    /// use kuzzle_sdk::types::KuzzleOptions;
    /// use kuzzle_sdk::protocols::Http;
    ///
    /// let http = Http::new(KuzzleOptions::new("localhost", 7512));
    /// ```
    pub fn new(options: KuzzleOptions) -> Http {
        Http {
            _client: Client::new(),
            _options: options,
            _routes: Http::read_routes_from_file(".http_routes.json"),
            _state: State::Offline,
        }
    }

    fn _get_route(&self, controller: &str, action: &str) -> Result<Route, Box<Error>> {
        match self._routes.get(controller) {
            Some(ctrl) => match ctrl.get(action) {
                Some(route) => Ok(route.clone()),
                None => Err(Box::new(SdkError::new(
                    "Http::_get_route",
                    &format!(
                        "Unable to find route for (controller/action) {}/{}",
                        controller, action
                    ),
                ))),
            },
            None => Err(Box::new(SdkError::new(
                "Http::_get_route",
                &format!(
                    "Unable to find route for (controller/action) {}/{}",
                    controller, action
                ),
            ))),
        }
    }

    fn read_routes_from_file(file: &str) -> Routes {
        let mut file = File::open(file).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Deserialize and print Rust data structure.
        let data: Routes = match serde_json::from_str(&contents) {
            Ok(json) => json,
            Err(err) => panic!("{}", err),
        };

        data
    }
}

use crate::protocols::Protocol;
use crate::types::{KuzzleRequest, KuzzleResponse, QueryOptions, SdkError};

use reqwest::{Client, Method, Url};
use std::error::Error;

#[cfg(test)]
use mockito;

impl Protocol for Http {
    fn is_ready(&self) -> bool {
        match self._state {
            State::Ready => true,
            State::Offline => false,
        }
    }

    fn close(&mut self) {
        self._state = State::Offline;
    }

    fn connect(&mut self) -> Result<(), Box<Error>> {
        if self._state == State::Ready {
            return Ok(());
        }

        #[cfg(not(test))]
        let url = Url::parse(&format!(
            "http://{}:{}",
            self._options.host(),
            self._options.port()
        ))?;

        #[cfg(test)]
        let url = &mockito::server_url();

        match reqwest::get(url) {
            Ok(_) => {
                self._state = State::Ready;
                Ok(())
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    fn send(
        &self,
        req: KuzzleRequest,
        _query_options: QueryOptions,
    ) -> Result<KuzzleResponse, Box<Error>> {
        if self._state == State::Offline {
            return Err(Box::new(SdkError::new(
                "Http::send",
                "Unable to execute request: not connected to a Kuzzle server.",
            )));
        }

        let kuzzle_route = self._get_route(req.controller(), req.action())?;
        let route = kuzzle_route
            .url
            .replace(":index", &req.index())
            .replace(":collection", &req.collection())
            .replace(":strategy", &req.strategy());

        #[cfg(not(test))]
        let host = &format!("http://{}:{}", self._options.host(), self._options.port(),);
        #[cfg(test)]
        let host = &mockito::server_url();

        let url: Url = Url::parse(&format!("{}{}", host, route))?;
        let method: Method = Method::from_bytes(kuzzle_route.verb.as_bytes())?;

        let mut request = self._client.request(method, url);

        if !req.body().is_empty() {
            request = request.json(&req.body());
        }

        if !req.query_strings().is_empty() {
            request = request.query(&req.query_strings());
        }

        if !req.jwt().is_empty() {
            request = request.bearer_auth(&req.jwt());
        }

        let mut raw_response = request.send()?;
        dbg!(&raw_response);
        let response : KuzzleResponse = raw_response.json()?;

        Ok(response)
    }

    fn once(&self) {
        unimplemented!();
    }

    fn listener_count(&self) {
        unimplemented!();
    }
}
