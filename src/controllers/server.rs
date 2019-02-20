use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions, SdkError};
use serde_json::{Map, Value};
use std::error::Error;

pub struct ServerController<'a>(pub &'a mut Kuzzle);

impl<'a> ServerController<'a> {
    /// Checks that an administrator account exists.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().admin_exists();
    ///
    /// ```
    ///
    pub fn admin_exists(&'a mut self) -> Result<bool, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("server", "adminExists");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res
                .result()
                .as_object()
                .unwrap()
                .get("exists")
                .unwrap()
                .as_bool()
                .unwrap()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Gets all stored internal statistic snapshots.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().get_all_stats();
    ///
    /// ```
    ///
    pub fn get_all_stats(&'a mut self) -> Result<Map<String, Value>, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("server", "getAllStats");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Returns the current Kuzzle configuration.
    ///
    /// This route should only be accessible to administrators,
    /// as it might return sensitive information about the backend.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().get_config();
    ///
    /// ```
    ///
    pub fn get_config(&'a mut self) -> Result<Map<String, Value>, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("server", "getConfig");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Returns the most recent statistics snapshot.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().get_last_stats();
    ///
    /// ```
    ///
    pub fn get_last_stats(&'a mut self) -> Result<Map<String, Value>, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("server", "getLastStats");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Returns statistics snapshots within a provided Epoch millis timestamp range.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().get_stats(1550444792010, 1550444805453);
    ///
    /// ```
    ///
    pub fn get_stats(&'a mut self, from: i64, to: i64) -> Result<Map<String, Value>, Box<Error>> {
        if from.to_string().len() != 13 || to.to_string().len() != 13 {
            return Err(Box::new(SdkError::new(
                "ServerController::get_stats",
                "`form` and `to` arguments need to be millis Epoch timestamps (13 digits).",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("server", "getStats")
            .add_to_query_strings("startTime", serde_json::to_value(from)?)
            .add_to_query_strings("stopTime", serde_json::to_value(to)?);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Returns information about Kuzzle: available API (base + extended), plugins,
    /// external services (Redis, Elasticsearch, ...), servers, etc.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().info();
    ///
    /// ```
    ///
    pub fn info(&'a mut self) -> Result<Map<String, Value>, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("server", "info");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Returns the current server timestamp, in Epoch-millis format.
    ///
    /// # Example
    ///
    /// ```
    /// use kuzzle_sdk::kuzzle::Kuzzle;
    /// use kuzzle_sdk::protocols::Http;
    /// use kuzzle_sdk::types::KuzzleOptions;
    ///
    /// let mut kuzzle = Kuzzle::new(
    ///     Http::new(
    ///         KuzzleOptions::new("localhost", 7512)
    ///     )
    /// );
    ///
    /// let res = kuzzle.server().info();
    ///
    /// ```
    ///
    pub fn now(&'a mut self) -> Result<u64, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("server", "now");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res
                .result()
                .as_object()
                .unwrap()
                .get("now")
                .unwrap()
                .as_u64()
                .unwrap()
                .clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocols::Http;
    use crate::types::KuzzleOptions;
    use mockito;

    #[test]
    fn admin_exists_ok() {
        let _m = mockito::mock("GET", "/_adminExists")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "adminExists",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                        "exists": true
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().admin_exists();

        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[test]
    fn admin_exists_fail_error() {
        let _m = mockito::mock("GET", "/_adminExists")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/adminExists] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/adminExists] for user -1\n"
                    },
		    "controller": "server",
		    "action": "adminExists",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().admin_exists();

        assert!(res.is_err());
    }

    #[test]
    fn get_all_stats_ok() {
        let _m = mockito::mock("GET", "/_getAllStats")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "getAllStats",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                      "total": 1,
                      "hits": [
                        {
                          "completedRequests": {
                            "websocket": 148,
                            "http": 24,
                            "mqtt": 78
                          },
                          "failedRequests": {
                            "websocket": 3
                          },
                          "ongoingRequests": {
                            "mqtt": 8,
                            "http": 2
                          },
                          "connections": {
                            "websocket": 13
                          },
                          "timestamp": 1453110641308
                        }
                      ]
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_all_stats();

        assert!(res.is_ok());
        let stats = res.unwrap();
        assert_eq!(stats.get("total").unwrap().as_u64().unwrap(), 1);
        let hits = stats.get("hits").unwrap().as_array().unwrap()[0]
            .as_object()
            .unwrap()
            .clone();
        assert_eq!(
            hits.get("timestamp").unwrap().as_u64().unwrap(),
            1453110641308
        );
        assert_eq!(
            hits.get("failedRequests")
                .unwrap()
                .as_object()
                .unwrap()
                .get("websocket")
                .unwrap()
                .as_u64()
                .unwrap(),
            3
        );
    }

    #[test]
    fn get_all_stats_fail_error() {
        let _m = mockito::mock("GET", "/_getAllStats")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/getAllStats] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/getAllStats] for user -1\n"
                    },
		    "controller": "server",
		    "action": "getAllStats",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_all_stats();

        assert!(res.is_err());
    }

    #[test]
    fn get_config_ok() {
        let _m = mockito::mock("GET", "/_getConfig")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "getConfig",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                      "limits": {
                        "concurrentRequests": 100,
                        "documentsFetchCount": 10000,
                        "documentsWriteCount": 200,
                        "requestsBufferSize": 50000,
                        "requestsBufferWarningThreshold": 5000,
                        "subscriptionConditionsCount": 16,
                        "subscriptionMinterms": 0,
                        "subscriptionRooms": 1000000,
                        "subscriptionDocumentTTL": 259200
                      },
                      "plugins": {
                        "common": {
                          "bootstrapLockTimeout": 5000,
                          "pipeWarnTime": 500,
                          "pipeTimeout": 5000,
                          "initTimeout": 10000
                        }
                      },
                      "version": "1.5.1"
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_config();

        assert!(res.is_ok());
        let config = res.unwrap();
        assert_eq!(config.get("version").unwrap().as_str().unwrap(), "1.5.1");
        assert_eq!(
            config
                .get("limits")
                .unwrap()
                .as_object()
                .unwrap()
                .get("concurrentRequests")
                .unwrap()
                .as_u64()
                .unwrap(),
            100
        );
    }

    #[test]
    fn get_config_fail_error() {
        let _m = mockito::mock("GET", "/_getConfig")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/getConfig] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/getConfig] for user -1\n"
                    },
		    "controller": "server",
		    "action": "getConfig",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_config();

        assert!(res.is_err());
    }

    #[test]
    fn get_last_stats_ok() {
        let _m = mockito::mock("GET", "/_getLastStats")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "getLastStats",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                      "completedRequests": {
                        "websocket": 148,
                        "http": 24,
                        "mqtt": 78
                      },
                      "failedRequests": {
                        "websocket": 3
                      },
                      "ongoingRequests": {
                        "mqtt": 8,
                        "http": 2
                      },
                      "connections": {
                        "websocket": 13
                      },
                      "timestamp": 1453110641308
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_last_stats();
        assert!(res.is_ok());
        let last_stats = res.unwrap();
        assert_eq!(
            last_stats.get("timestamp").unwrap().as_u64().unwrap(),
            1453110641308
        );
        assert_eq!(
            last_stats
                .get("failedRequests")
                .unwrap()
                .as_object()
                .unwrap()
                .get("websocket")
                .unwrap()
                .as_u64()
                .unwrap(),
            3
        );
    }

    #[test]
    fn get_last_stats_fail_error() {
        let _m = mockito::mock("GET", "/_getLastStats")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/getLastStats] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/getLastStats] for user -1\n"
                    },
		    "controller": "server",
		    "action": "getLastStats",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_last_stats();

        assert!(res.is_err());
    }

    #[test]
    fn get_stats_ok() {
        let _m = mockito::mock(
            "GET", 
            mockito::Matcher::Regex(r"^/_getStats\?(startTime|stopTime)=[0-9]{13}\&(startTime|stopTime)=[0-9]{13}$".to_string())
        )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "getStats",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                      "completedRequests": {
                        "websocket": 148,
                        "http": 24,
                        "mqtt": 78
                      },
                      "failedRequests": {
                        "websocket": 3
                      },
                      "ongoingRequests": {
                        "mqtt": 8,
                        "http": 2
                      },
                      "connections": {
                        "websocket": 13
                      },
                      "timestamp": 1453110641308
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_stats(1550439618398, 1550436918273);
        assert!(res.is_ok());
        let stats = res.unwrap();
        assert_eq!(
            stats.get("timestamp").unwrap().as_u64().unwrap(),
            1453110641308
        );
        assert_eq!(
            stats
                .get("failedRequests")
                .unwrap()
                .as_object()
                .unwrap()
                .get("websocket")
                .unwrap()
                .as_u64()
                .unwrap(),
            3
        );
    }

    #[test]
    fn get_stats_fail_error() {
        let _m = mockito::mock(
            "GET", 
            mockito::Matcher::Regex(r"^/_getStats\?(startTime|stopTime)=[0-9]{13}\&(startTime|stopTime)=[0-9]{13}$".to_string())
        )
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/getStats] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/getStats] for user -1\n"
                    },
		    "controller": "server",
		    "action": "getStats",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_stats(1550439618398, 1550436918273);

        assert!(res.is_err());
    }

    #[test]
    fn get_stats_fail_all_bad_timestamp_format() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_stats(1550439618, 150436918273);

        assert!(res.is_err());
    }

    #[test]
    fn get_stats_fail_one_bad_timestamp_format() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().get_stats(155043961845, 150436918273);

        assert!(res.is_err());
    }

    #[test]
    fn info_ok() {
        let _m = mockito::mock("GET", "/_serverInfo")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "info",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                      "serverInfo": {
                        "kuzzle": {
                          "api": {
                            "routes": {
                              "auth": {
                                "login": {
                                  "controller": "auth",
                                  "action": "login",
                                  "http": [
                                    {
                                        "url": "/_login/:strategy",
                                        "verb": "GET"
                                    },
                                    {
                                        "url": "/_login/:strategy",
                                        "verb": "POST"
                                    }
                                  ]
                                }
                              }
                            }
                          },
                          "memoryUsed": 115036160,
                          "nodeVersion": "v8.9.0",
                          "plugins": {
                            "kuzzle-plugin-auth-passport-local": {
                              "manifest": {
                                "name": "kuzzle-plugin-auth-passport-local",
                                "path": "/var/app/plugins/enabled/kuzzle-plugin-auth-passport-local",
                                "kuzzleVersion": ">=1.0.0 <2.0.0"
                              },
                              "hooks": [],
                              "pipes": [],
                              "controllers": [],
                              "routes": [],
                              "strategies": [ "local" ]
                            }
                          }
                        } 
                      }
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().info();
        assert!(res.is_ok());
        let info = res.unwrap();
        assert_eq!(
            info.get("serverInfo")
                .unwrap()
                .as_object()
                .unwrap()
                .get("kuzzle")
                .unwrap()
                .as_object()
                .unwrap()
                .get("memoryUsed")
                .unwrap()
                .as_u64()
                .unwrap(),
            115036160
        );
    }

    #[test]
    fn info_fail_error() {
        let _m = mockito::mock("GET", "/_serverInfo")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/info] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/info] for user -1\n"
                    },
		    "controller": "server",
		    "action": "info",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().info();

        assert!(res.is_err());
    }

    #[test]
    fn now_ok() {
        let _m = mockito::mock("GET", "/_now")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "server",
		    "action": "adminExists",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                        "now": 1928374619383
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().now();

        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_string().len(), 13);
    }

    #[test]
    fn now_fail_error() {
        let _m = mockito::mock("GET", "/_now")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/server/now] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/server/now] for user -1\n"
                    },
		    "controller": "server",
		    "action": "now",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.server().now();

        assert!(res.is_err());
    }
}
