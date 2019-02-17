use std::collections::HashMap;

type Routes = HashMap<String, HashMap<String, Route>>;

#[derive(Deserialize, Clone)]
pub struct Route {
    pub url: String,
    pub verb: String,
}

use crate::types::KuzzleOptions;

pub struct Http {
    _client: Client,
    _options: KuzzleOptions,
    _routes: Routes,
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
        }
    }

    fn _get_route(&self, controller: &str, action: &str) -> Route {
        self._routes
            .get(controller)
            .unwrap()
            .get(action)
            .unwrap()
            .clone()
    }

    fn read_routes_from_file(file: &str) -> Routes {
        let mut file = match File::open(file) {
            Ok(fd) => fd,
            Err(err) => panic!("{}", err),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        };

        // Deserialize and print Rust data structure.
        let data: Routes = match serde_json::from_str(&contents) {
            Ok(json) => json,
            Err(err) => panic!("{}", err),
        };

        data
    }
}

use crate::protocols::Protocol;
use crate::types::{KuzzleRequest, KuzzleResponse, QueryOptions};

use reqwest::{Client, Method, Url};
use std::error::Error;

#[cfg(test)]
use mockito;

impl Protocol for Http {
    fn once(&self) {
        unimplemented!();
    }
    fn listener_count(&self) {
        unimplemented!();
    }
    fn connect(&self) {
        unimplemented!();
    }
    fn send(
        &self,
        req: KuzzleRequest,
        _query_options: QueryOptions,
    ) -> Result<KuzzleResponse, Box<Error>> {
        let kuzzle_route = self._get_route(req.controller(), req.action());
        let route = kuzzle_route
            .url
            .replace(":index", &req.index().clone().unwrap_or(String::new()))
            .replace(
                ":collection",
                &req.collection().clone().unwrap_or(String::new()),
            );

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

        let response: KuzzleResponse = request.send()?.json()?;
        Ok(response)
    }
    fn close(&self) {
        unimplemented!();
    }
    fn state(&self) {
        unimplemented!();
    }
    fn request_history(&self) {
        unimplemented!();
    }
    fn start_queuing(&self) {
        unimplemented!();
    }
    fn stop_queuing(&self) {
        unimplemented!();
    }
    fn clear_queue(&self) {
        unimplemented!();
    }
}
