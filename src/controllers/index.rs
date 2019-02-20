use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions, SdkError};
use serde_json;

use std::error::Error;

pub struct IndexController<'a>(pub &'a mut Kuzzle);

impl<'a> IndexController<'a> {
    /// Create a new index in Kuzzl.
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
    /// let res = kuzzle.index().create("ferris_index");
    ///
    /// ```
    ///
    pub fn create(&'a mut self, index: &str) -> Result<(), Box<Error>> {
        if index.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::create",
                "index argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "create").set_index(index);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Delete an entire data index from Kuzzle.
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
    /// let res = kuzzle.index().delete("ferris_index");
    ///
    /// ```
    ///
    pub fn delete(&'a mut self, index: &str) -> Result<(), Box<Error>> {
        if index.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::delete",
                "index argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "delete").set_index(index);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Checks if the given index exists in Kuzzle.
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
    /// let res = kuzzle.index().exists("ferris_index");
    ///
    /// ```
    ///
    pub fn exists(&'a mut self, index: &str) -> Result<bool, Box<Error>> {
        if index.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::exists",
                "index argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "exists").set_index(index);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_bool().unwrap()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Return the current autorefresh status for the index.
    /// Each index has an autorefresh flag. When set to true, each write request trigger
    /// a refresh action on Elasticsearch. Without a refresh after a write request,
    /// the documents may not be immediately visible in search.
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
    /// let res = kuzzle.index().get_auto_refresh("ferris_index");
    ///
    /// ```
    ///
    pub fn get_auto_refresh(&'a mut self, index: &str) -> Result<bool, Box<Error>> {
        if index.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::get_auto_refresh",
                "index argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "getAutoRefresh").set_index(index);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res.result().as_bool().unwrap()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Get the complete list of data indexes handled by Kuzzle.
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
    /// let res = kuzzle.index().list();
    ///
    /// ```
    ///
    pub fn list(&'a mut self) -> Result<Vec<String>, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("index", "list");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res
                .result()
                .as_object()
                .unwrap()
                .get("indexes")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Deletes multiple indexes at once.
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
    /// let res = kuzzle.index().list();
    ///
    /// ```
    ///
    pub fn mdelete(&'a mut self, indexes: Vec<String>) -> Result<Vec<String>, Box<Error>> {
        if indexes.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::mDelete",
                "indexes argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "mDelete")
            .add_to_body("indexes", serde_json::to_value(indexes)?);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(res
                .result()
                .as_object()
                .unwrap()
                .get("deleted")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Forces an immediate reindexation of the provided index.
    /// When writing or deleting documents in Kuzzle, the changes need to be
    /// indexed before being reflected in the search results. By default,
    /// this operation can take up to 1 second.
    ///
    /// Note: forcing immediate refreshes comes with performance costs,
    /// and should only performed when absolutely necessary.
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
    /// let res = kuzzle.index().refresh("ferris_index");
    ///
    /// ```
    ///
    pub fn refresh(&'a mut self, index: &str) -> Result<(), Box<Error>> {
        if index.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::refresh",
                "index argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "refresh").set_index(index);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Forces an immediate reindexation of Kuzzle internal storage.
    /// When writing or deleting security documents in Kuzzle (users, profiles, roles, and so on),
    /// the changes need to be indexed before being reflected in the search results.
    /// By default, this operation can take up to 1 second.
    ///
    /// Note: forcing immediate refreshes comes with performance costs,
    /// and should only performed when absolutely necessary.
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
    /// let res = kuzzle.index().refresh_internal();
    ///
    /// ```
    ///
    pub fn refresh_internal(&'a mut self) -> Result<(), Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("index", "refreshInternal");
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    /// Changes the autoRefresh configuration of an index.
    /// The autoRefresh flag, when set to true, tells Kuzzle to perform
    /// an immediate refresh request after each write request, instead of waiting the regular
    /// refreshes occuring every seconds.
    ///
    /// Note: refreshes come with performance costs. Set the autoRefresh flag to true only for
    /// indexes needing changes to be immediately available through searches,
    /// and only for slowly changing indexes.
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
    /// let res = kuzzle.index().set_auto_refresh("ferris_index", true);
    ///
    /// ```
    ///
    pub fn set_auto_refresh(
        &'a mut self,
        index: &str,
        auto_refresh: bool,
    ) -> Result<(), Box<Error>> {
        if index.is_empty() {
            return Err(Box::new(SdkError::new(
                "IndexController::set_auto_refresh",
                "index argument must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("index", "setAutoRefresh")
            .set_index(index)
            .add_to_body("autoRefresh", serde_json::to_value(auto_refresh)?);
        let res = self.kuzzle().query(req, QueryOptions::new())?;
        match &res.error() {
            None => Ok(()),
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
    fn create_ok() {
        let _m = mockito::mock("POST", "/ferris_index/_create")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "index",
		    "action": "create",
		    "collection": null,
		    "index": "ferris_index",
		    "volatile": null,
                    "result": {
                        "acknowledged": true,
                        "shards_acknowledged": true
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().create("ferris_index");

        assert!(res.is_ok());
    }

    #[test]
    fn create_fail_already_exists() {
        let _m = mockito::mock("POST", "/ferris_index/_create")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "c6fd04c1-45d0-48ef-9eed-ef95c4a97422",
                    "status": 400,
                    "error": {
                        "message": "index [ferris_index/gpBwiLwFTfu8E5mV37UZEQ] already exists",
                        "status": 400,
                        "stack": "BadRequestError: index [ferris_index/gpBwiLwFTfu8E5mV37UZEQ] already exists\n"
                    },
                    "controller": "index",
                    "action": "create",
                    "collection": null,
                    "index": "ferris_index",
                    "volatile": null,
                    "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().create("ferris_index");

        assert!(res.is_err());
    }

    #[test]
    fn create_fail_empty_index_name() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().create("");

        assert!(res.is_err());
    }

    #[test]
    fn delete_ok() {
        let _m = mockito::mock("DELETE", "/ferris_index")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "delete",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": {
                        "acknowledged": true
                      }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().delete("ferris_index");

        assert!(res.is_ok());
    }

    #[test]
    fn delete_fail_not_found_index() {
        let _m = mockito::mock("DELETE", "/ferris_index")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 404,
                      "error": {
                        "message": "Index \"\"ferris_index\"\" does not exist, please create it first",
                        "status": 404,
                        "stack": "NotFoundError: Index \"\"ferris_index\"\" does not exist, please create it first\n"
                      },
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "delete",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().delete("ferris_index");

        assert!(res.is_err());
    }

    #[test]
    fn delete_fail_empty_index_name() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().delete("");

        assert!(res.is_err());
    }

    #[test]
    fn exists_ok_true() {
        let _m = mockito::mock("GET", "/ferris_index/_exists")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "exists",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": true
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().exists("ferris_index");

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), true);
    }

    #[test]
    fn exists_ok_false() {
        let _m = mockito::mock("GET", "/ferris_index/_exists")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "exists",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": false
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().exists("ferris_index");

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), false);
    }

    #[test]
    fn exists_fail_error() {
        let _m = mockito::mock("GET", "/ferris_index/_exists")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 403,
                      "error": {
                        "message": "Forbidden action [ferris_index/null/index/exists] for user -1",
                        "status": 403,
                        "stack": "ForbiddenError: Forbidden action [ferris_index/null/index/exists] for user -1\n    at new ForbiddenError (/var/app/node_modules/kuzzle-common-objects/lib/errors/forbiddenError.js:5:5)\n    at kuzzle.repositories.token.verifyToken.then.then.then.isAllowed (/var/app/lib/api/controllers/funnelController.js:337:21)\n    at tryCatcher (/var/app/node_modules/bluebird/js/release/util.js:16:23)\n    at Promise._settlePromiseFromHandler (/var/app/node_modules/bluebird/js/release/promise.js:512:31)\n    at Promise._settlePromise (/var/app/node_modules/bluebird/js/release/promise.js:569:18)\n    at Promise._settlePromise0 (/var/app/node_modules/bluebird/js/release/promise.js:614:10)\n    at Promise._settlePromises (/var/app/node_modules/bluebird/js/release/promise.js:694:18)\n    at _drainQueueStep (/var/app/node_modules/bluebird/js/release/async.js:138:12)\n    at _drainQueue (/var/app/node_modules/bluebird/js/release/async.js:131:9)\n    at Async._drainQueues (/var/app/node_modules/bluebird/js/release/async.js:147:5)\n    at Immediate.Async.drainQueues (/var/app/node_modules/bluebird/js/release/async.js:17:14)\n    at runCallback (timers.js:810:20)\n    at tryOnImmediate (timers.js:768:5)\n    at processImmediate [as _immediateCallback] (timers.js:745:5)"
                      },
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "exists",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().exists("ferris_index");

        assert!(res.is_err());
    }

    #[test]
    fn exists_fail_empty_index_name() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().exists("");

        assert!(res.is_err());
    }

    #[test]
    fn get_auto_refresh_ok_true() {
        let _m = mockito::mock("GET", "/ferris_index/_autoRefresh")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "getAutoRefresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": true
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().get_auto_refresh("ferris_index");

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), true);
    }

    #[test]
    fn get_auto_refresh_ok_false() {
        let _m = mockito::mock("GET", "/ferris_index/_autoRefresh")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "getAutoRefresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": false
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().get_auto_refresh("ferris_index");

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), false);
    }

    #[test]
    fn get_auto_refresh_fail_error() {
        let _m = mockito::mock("GET", "/ferris_index/_autoRefresh")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 403,
                      "error": {
                        "message": "Forbidden action [ferris_index/null/index/getAutoRefresh] for user -1",
                        "status": 403,
                        "stack": "ForbiddenError: Forbidden action [ferris_index/null/index/getAutoRefresh] for user -1\n"
                      },
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "getAutoRefresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().get_auto_refresh("ferris_index");

        assert!(res.is_err());
    }

    #[test]
    fn get_auto_refresh_fail_empty_index_name() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().get_auto_refresh("");

        assert!(res.is_err());
    }

    #[test]
    fn list_ok() {
        let _m = mockito::mock("GET", "/_list")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "index",
		    "action": "list",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                        "indexes": [
                            "ferris_the_crab",
                            "ferris_the_happy_crab"
                        ]
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().list();

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 2);
    }

    #[test]
    fn list_fail_error() {
        let _m = mockito::mock("GET", "/_list")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "c6fd04c1-45d0-48ef-9eed-ef95c4a97422",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/index/list] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/index/list] for user -1\n"
                    },
                    "controller": "index",
                    "action": "list",
                    "collection": null,
                    "index": null,
                    "volatile": null,
                    "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().list();

        assert!(res.is_err());
    }

    #[test]
    fn mdelete_ok() {
        let _m = mockito::mock("DELETE", "/_mdelete")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
		    "error": null,
		    "controller": "index",
		    "action": "mDelete",
		    "collection": null,
		    "index": null,
		    "volatile": null,
                    "result": {
                        "deleted": [
                            "ferris_the_crab",
                            "ferris_the_happy_crab"
                        ]
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().mdelete(vec![
            "ferris_the_crab".to_string(),
            "ferris_the_happy_crab".to_string(),
        ]);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 2);
    }

    #[test]
    fn mdelete_fail_error() {
        let _m = mockito::mock("DELETE", "/_mdelete")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "c6fd04c1-45d0-48ef-9eed-ef95c4a97422",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [_mDelete/null/index/delete] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [_mDelete/null/index/delete] for user -1\n"
                    },
                    "controller": "index",
                    "action": "mDelete",
                    "collection": null,
                    "index": null,
                    "volatile": null,
                    "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().mdelete(vec!["ferris_lair".to_string()]);

        assert!(res.is_err());
    }

    #[test]
    fn mdelete_fail_not_found() {
        let _m = mockito::mock("DELETE", "/_mdelete")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "c6fd04c1-45d0-48ef-9eed-ef95c4a97422",
                    "status": 200,
                    "error": null,
                    "controller": "index",
                    "action": "mDelete",
                    "collection": null,
                    "index": null,
                    "volatile": null,
                    "result": {
                        "deleted": []
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().mdelete(vec!["ferris_not_found".to_string()]);

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 0);
    }

    #[test]
    fn mdelete_fail_empty_indexes_array() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().mdelete(vec![]);

        assert!(res.is_err());
    }

    #[test]
    fn refresh_ok() {
        let _m = mockito::mock("POST", "/ferris_index/_refresh")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "refresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": {
                        "_shards": {
                            "failed": 0,
                            "succressful": 5,
                            "total": 10
                        }
                      }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().refresh("ferris_index");

        assert!(res.is_ok());
    }

    #[test]
    fn refresh_fail_not_found_index() {
        let _m = mockito::mock("POST", "/ferris_index/_refresh")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 404,
                      "error": {
                        "message": "Index \"\"ferris_index\"\" does not exist, please create it first",
                        "status": 404,
                        "stack": "NotFoundError: Index \"\"ferris_index\"\" does not exist, please create it first\n"
                      },
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "refresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().refresh("ferris_index");

        assert!(res.is_err());
    }

    #[test]
    fn refresh_fail_empty_index_name() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().refresh("");

        assert!(res.is_err());
    }

    #[test]
    fn refresh_internal_ok() {
        let _m = mockito::mock("POST", "/_refreshInternal")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": null,
                      "controller": "index",
                      "action": "refreshInternal",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": {
                        "acknowledged": true
                      }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().refresh_internal();

        assert!(res.is_ok());
    }

    #[test]
    fn refresh_internal_fail_error() {
        let _m = mockito::mock("POST", "/_refreshInternal")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 403,
                      "error": {
                        "message": "Forbidden action [null/null/index/refreshInternal] for user -1",
                        "status": 403,
                        "stack": "ForbiddenError: Forbidden action [null/null/index/refreshInternal] for user -1\n"
                      },
                      "index": null,
                      "controller": "index",
                      "action": "refreshInternal",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().refresh_internal();

        assert!(res.is_err());
    }

    #[test]
    fn set_auto_refresh_ok() {
        let _m = mockito::mock("POST", "/ferris_index/_autoRefresh")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 200,
                      "error": null,
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "setAutoRefresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": {
                        "response": true
                      }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().set_auto_refresh("ferris_index", true);

        assert!(res.is_ok());
    }

    #[test]
    fn set_auto_refresh_fail_not_found_index() {
        let _m = mockito::mock("POST", "/ferris_index/_autoRefresh")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                      "status": 404,
                      "error": {
                        "message": "Index \"\"ferris_index\"\" does not exist, please create it first",
                        "status": 404,
                        "stack": "NotFoundError: Index \"\"ferris_index\"\" does not exist, please create it first\n"
                      },
                      "index": "ferris_index",
                      "controller": "index",
                      "action": "setAutoRefresh",
                      "requestId": "29d98f35-8cfd-4eeb-97fd-f135d931f0bd",
                      "result": null
                }"#,
            ).create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().set_auto_refresh("ferris_index", true);

        assert!(res.is_err());
    }

    #[test]
    fn set_auto_refresh_fail_empty_index_name() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.index().set_auto_refresh("", true);

        assert!(res.is_err());
    }
}
