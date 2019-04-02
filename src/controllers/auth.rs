use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions, SdkError, TokenValidity, User, UserRight};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::error::Error;

pub struct AuthController<'a>(pub &'a mut Kuzzle);

impl<'a> AuthController<'a> {
    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }

    pub fn check_token(&'a mut self, token: &str) -> Result<TokenValidity, Box<Error>> {
        if token.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::check_token",
                "token argument must not be empty.",
            )));
        }
        let req: KuzzleRequest = KuzzleRequest::new("auth", "checkToken")
            .add_to_body("token", serde_json::to_value(token)?);
        let response = self.kuzzle().query(req, QueryOptions::new())?;
        match &response.error() {
            None => {
                let token_validity: TokenValidity =
                    serde_json::from_value(response.result().clone())?;
                Ok(token_validity)
            }
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn create_my_credentials(
        &'a mut self,
        strategy: &str,
        username: &str,
        password: &str,
    ) -> Result<Map<String, Value>, Box<Error>> {
        if strategy.is_empty() || username.is_empty() || password.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::create_my_credentials",
                "strategy, username and password arguments must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::create_my_credentials",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("auth", "createMyCredentials")
            .set_strategy(strategy)
            .add_to_body("username", serde_json::to_value(username)?)
            .add_to_body("password", serde_json::to_value(password)?);

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => Ok(response.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn credentials_exist(&'a mut self, strategy: &str) -> Result<bool, Box<Error>> {
        if strategy.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::credentials_exist",
                "strategy argument must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::credentials_exist",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest =
            KuzzleRequest::new("auth", "credentialsExist").set_strategy(strategy);

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => Ok(response.result().as_bool().unwrap()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn delete_my_credentials(&'a mut self, strategy: &str) -> Result<bool, Box<Error>> {
        if strategy.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::delete_my_credentials",
                "strategy argument must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::delete_my_credentials",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest =
            KuzzleRequest::new("auth", "deleteMyCredentials").set_strategy(strategy);

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => Ok(response
                .result()
                .get("acknowledged")
                .unwrap()
                .as_bool()
                .unwrap()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn get_current_user(&'a mut self, strategy: &str) -> Result<User, Box<Error>> {
        if strategy.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::get_current_user",
                "strategy argument must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::get_current_user",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest =
            KuzzleRequest::new("auth", "getCurrentUser").set_strategy(strategy);

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => {
                let user: User = serde_json::from_value(response.result().clone())?;
                Ok(user)
            }
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn get_my_credentials(
        &'a mut self,
        strategy: &str,
    ) -> Result<Map<String, Value>, Box<Error>> {
        if strategy.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::get_my_credentials",
                "strategy argument must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::get_my_credentials",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest =
            KuzzleRequest::new("auth", "getMyCredentials").set_strategy(strategy);

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => Ok(response.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn get_my_rights(&'a mut self, strategy: &str) -> Result<Vec<UserRight>, Box<Error>> {
        if strategy.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::get_my_rights",
                "strategy argument must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::get_my_rights",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("auth", "getMyRights").set_strategy(strategy);

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => {
                let user_rights: Vec<UserRight> = response
                    .result()
                    .as_object()
                    .unwrap()
                    .get("hits")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| serde_json::from_value(x.clone()).unwrap())
                    .collect::<Vec<_>>();

                Ok(user_rights)
            }
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn get_strategies(&'a mut self) -> Result<Vec<String>, Box<Error>> {
        let req: KuzzleRequest = KuzzleRequest::new("auth", "getStrategies");

        let response = self.kuzzle().query(req, QueryOptions::new())?;
        match &response.error() {
            None => {
                let raw_strats = response.result().as_array().unwrap().clone();

                let strategies: Vec<String> = raw_strats
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect::<Vec<String>>();;

                Ok(strategies)
            }
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn login(
        &'a mut self,
        strategy: &str,
        username: &str,
        password: &str,
    ) -> Result<String, Box<Error>> {
        if strategy.is_empty() || username.is_empty() || password.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::login",
                "strategy, username and password arguments must not be empty.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("auth", "login")
            .set_strategy(strategy)
            .add_to_body("username", serde_json::to_value(username)?)
            .add_to_body("password", serde_json::to_value(password)?);

        let kuzzle = self.kuzzle();
        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => {
                let jwt = response
                    .result()
                    .as_object()
                    .unwrap()
                    .get("jwt")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                kuzzle.set_jwt(jwt);
                Ok(kuzzle.jwt())
            }
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn logout(&'a mut self) -> Result<(), Box<Error>> {
        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::logout",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("auth", "logout");

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => {
                // TODO: Unsubscribe all when websocket will be implemented
                kuzzle.set_jwt("".to_string());
                Ok(())
            }
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }

    pub fn update_my_credentials(
        &'a mut self,
        strategy: &str,
        content: &'a HashMap<String, Value>,
    ) -> Result<Map<String, Value>, Box<Error>> {
        if content.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::update_my_credentials",
                "content argument must not be empty.",
            )));
        }

        if strategy.is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::update_my_credentials",
                "strategy argument must not be empty.",
            )));
        }

        let kuzzle = self.kuzzle();
        if kuzzle.jwt().clone().is_empty() {
            return Err(Box::new(SdkError::new(
                "AuthController::update_my_credentials",
                "You need to be logged in to use this function.",
            )));
        }

        let req: KuzzleRequest = KuzzleRequest::new("auth", "updateMyCredentials")
            .set_strategy(strategy)
            .set_body(content.clone());

        let response = kuzzle.query(req, QueryOptions::new())?;
        match &response.error() {
            None => Ok(response.result().as_object().unwrap().clone()),
            Some(k_err) => Err(Box::new(k_err.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocols::Http;
    use crate::types::KuzzleOptions;
    use mockito;

    #[test]
    fn check_token_ok() {
        let _m = mockito::mock("POST", "/_checkToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "checkToken",
                    "volatile": {},
                    "result": {
                      "valid": true,
                      "expiresAt": 1538557452248
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().check_token("veryCoolAndLongToken");

        assert!(res.is_ok());
        let token_validity = res.unwrap();
        assert!(token_validity.valid());
        assert_eq!(token_validity.expires_at(), 1538557452248);
    }

    #[test]
    fn check_token_ok_not_valid() {
        let _m = mockito::mock("POST", "/_checkToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "checkToken",
                    "volatile": {},
                    "result": {
                      "valid": false,
                      "state": "Json Web Token Error"
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().check_token("veryCoolAndLongToken");

        assert!(res.is_ok());
        let token_validity = res.unwrap();
        assert!(!token_validity.valid());
        assert_eq!(token_validity.state(), "Json Web Token Error".to_string());
    }

    #[test]
    fn check_token_fail_error() {
        let _m = mockito::mock("POST", "/_checkToken")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/auth/checkToken] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/checkToken] for user -1\n"
                    },
                    "controller": "auth",
                    "action": "checkToken",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().check_token("veryCoolAndLongToken");

        assert!(res.is_err());
    }

    #[test]
    fn check_token_fail_empty_arg() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().check_token("");

        assert!(res.is_err());
    }

    #[test]
    fn create_my_credentials_ok() {
        let _m = mockito::mock("POST", "/credentials/local/_me/_create")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "createMyCredentials",
                    "volatile": {},
                    "result": {
                      "username": "newUser",
                      "kuid": "myNewUserKUID"
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().create_my_credentials("local", "admin", "password");

        assert!(&res.is_ok());
        assert_eq!(
            &res.unwrap().get("username").unwrap().as_str().unwrap(),
            &"newUser"
        );
    }

    #[test]
    fn create_my_credentials_fail_error() {
        let _m = mockito::mock("POST", "/credentials/local/_me/_create")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(412)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 412,
                    "error": {
                      "message": "A strategy already exists for user \"-1\".",
                      "status": 412,
                      "stack": "Precondition Failed: A strategy already exists for user \"-1\".\n    at new PreconditionError (/var/app/node_modules/kuzzle-common-objects/lib/errors/preconditionError.js:5:5)\n    at exists.then.exists (/var/app/plugins/available/kuzzle-plugin-auth-passport-local/lib/index.js:224:17)\n    at tryCatcher (/var/app/node_modules/bluebird/js/release/util.js:16:23)\n    at Promise._settlePromiseFromHandler (/var/app/node_modules/bluebird/js/release/promise.js:512:31)\n    at Promise._settlePromise (/var/app/node_modules/bluebird/js/release/promise.js:569:18)\n    at Promise._settlePromise0 (/var/app/node_modules/bluebird/js/release/promise.js:614:10)\n    at Promise._settlePromises (/var/app/node_modules/bluebird/js/release/promise.js:694:18)\n    at _drainQueueStep (/var/app/node_modules/bluebird/js/release/async.js:138:12)\n    at _drainQueue (/var/app/node_modules/bluebird/js/release/async.js:131:9)\n    at Async._drainQueues (/var/app/node_modules/bluebird/js/release/async.js:147:5)\n    at Immediate.Async.drainQueues (/var/app/node_modules/bluebird/js/release/async.js:17:14)\n    at runCallback (timers.js:810:20)\n    at tryOnImmediate (timers.js:768:5)\n    at processImmediate [as _immediateCallback] (timers.js:745:5)"
                    },
                    "controller": "auth",
                    "action": "createMyCredentials",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().create_my_credentials("local", "admin", "password");

        assert!(res.is_err());
    }

    #[test]
    fn create_my_credentials_fail_empty_args() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().create_my_credentials("local", "", "");

        assert!(res.is_err());
    }

    #[test]
    fn create_my_credentials_fail_not_logged_in() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k
            .auth()
            .create_my_credentials("local", "username", "password");

        assert!(res.is_err());
    }

    #[test]
    fn credentials_exist_ok() {
        let _m = mockito::mock("GET", "/credentials/local/_me/_exists")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "credentialsExist",
                    "volatile": {},
                    "result": true
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().credentials_exist("local");

        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[test]
    fn credentials_exist_fail_error() {
        let _m = mockito::mock("GET", "/credentials/local/_me/_exists")
            .match_header("authorization", "Bearer mySuperBadToken")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": {                                                                                                                                                                                                                              
                      "message": "Forbidden action [null/null/auth/credentialsExist] for user -1",                                                                                                                                                          
                      "status": 403,                                                                                                                                                                                                                        
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/credentialsExist] for user -1\n"                                                                                                       
                    }, 
                    "controller": "auth",
                    "action": "credentialsExist",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperBadToken".to_string());
        let res = k.auth().credentials_exist("local");

        assert!(res.is_err());
    }

    #[test]
    fn credentials_exist_fail_empty_strategy() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().credentials_exist("");

        assert!(res.is_err());
    }

    #[test]
    fn credentials_exist_fail_no_jwt() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().credentials_exist("local");

        assert!(res.is_err());
    }

    #[test]
    fn delete_my_credentials_ok() {
        let _m = mockito::mock("DELETE", "/credentials/local/_me")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "deleteMyCredentials",
                    "volatile": {},
                    "result": {
                      "acknowledged": true
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().delete_my_credentials("local");

        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[test]
    fn delete_my_credentials_fail_error() {
        let _m = mockito::mock("DELETE", "/credentials/local/_me")
            .match_header("authorization", "Bearer mySuperBadToken")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {                                                                                                                                                                                                                              
                      "message": "Forbidden action [null/null/auth/deleteMyCredentials] for user -1",                                                                                                                                                          
                      "status": 403,                                                                                                                                                                                                                        
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/deleteMyCredentials] for user -1\n"                                                                                                       
                    }, 
                    "controller": "auth",
                    "action": "deleteMyCredentials",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperBadToken".to_string());
        let res = k.auth().delete_my_credentials("local");

        assert!(res.is_err());
    }

    #[test]
    fn delete_my_credentials_fail_empty_strategy() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().delete_my_credentials("");

        assert!(res.is_err());
    }

    #[test]
    fn delete_my_credentials_fail_no_jwt() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().delete_my_credentials("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_current_user_ok() {
        let _m = mockito::mock("GET", "/users/_me")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "getCurrentUser",
                    "volatile": {},
                    "result": {
                      "_id": "kuid",
                      "_source": {
                        "profileIds": ["customProfile1", "customProfile2"],
                        "name": {
                          "first": "Steve",
                          "last": "Wozniak"
                        }
                      },
                      "strategies": ["local"]
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().get_current_user("local");

        assert!(res.is_ok());
        let user: User = res.unwrap();
        assert_eq!(user.id(), "kuid".to_string());
        assert_eq!(user.strategies()[0], "local".to_string());
    }

    #[test]
    fn get_current_user_fail_error() {
        let _m = mockito::mock("GET", "/users/_me")
            .match_header("authorization", "Bearer mySuperBadToken")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/auth/getCurrentUser] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/getCurrentUser] for user -1\n"
                    },
                    "controller": "auth",
                    "action": "getCurrentUser",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperBadToken".to_string());
        let res = k.auth().get_current_user("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_current_user_fail_empty_strategy() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().get_current_user("");

        assert!(res.is_err());
    }

    #[test]
    fn get_current_user_fail_no_jwt() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().get_current_user("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_my_credentials_ok() {
        let _m = mockito::mock("GET", "/credentials/local/_me")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "getMyCredentials",
                    "volatile": {},
                    "result": {
                      "username": "MyUser",
                      "kuid": "kuid"
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().get_my_credentials("local");

        assert!(res.is_ok());
        let response = res.unwrap();
        assert_eq!(
            response.get("username").unwrap().as_str().unwrap(),
            "MyUser"
        );
        assert_eq!(response.get("kuid").unwrap().as_str().unwrap(), "kuid");
    }

    #[test]
    fn get_my_credentials_fail_error() {
        let _m = mockito::mock("GET", "/credentials/local/_me")
            .match_header("authorization", "Bearer mySuperBadToken")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/auth/getMyCredentials] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/getMyCredentials] for user -1\n"
                    },
                    "controller": "auth",
                    "action": "getMyCredentials",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperBadToken".to_string());
        let res = k.auth().get_my_credentials("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_my_credentials_fail_empty_strategy() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().get_my_credentials("");

        assert!(res.is_err());
    }

    #[test]
    fn get_my_credentials_fail_no_jwt() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().get_my_credentials("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_my_rights_ok() {
        let _m = mockito::mock("GET", "/users/_me/_rights")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "getMyRights",
                    "volatile": {},
                    "result": {
                      "hits": [
                        {
                          "controller": "document",
                          "action": "get",
                          "index": "foo",
                          "collection": "bar",
                          "value": "allowed"
                        },
                        {
                          "controller": "document",
                          "action": "search",
                          "index": "foo",
                          "collection": "bar",
                          "value": "allowed"
                        },
                        {
                          "controller": "document",
                          "action": "write",
                          "index": "foo",
                          "collection": "bar",
                          "value": "denied"
                        }
                      ]
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().get_my_rights("local");

        assert!(res.is_ok());
        let my_rights = res.unwrap();
        assert_eq!(my_rights.len(), 3);
    }

    #[test]
    fn get_my_rights_fail_error() {
        let _m = mockito::mock("GET", "/users/_me/_rights")
            .match_header("authorization", "Bearer mySuperBadToken")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/auth/getMyRights] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/getMyRights] for user -1\n"
                    },
                    "controller": "auth",
                    "action": "getMyRights",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperBadToken".to_string());
        let res = k.auth().get_my_rights("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_my_rights_fail_empty_strategy() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().get_my_rights("");

        assert!(res.is_err());
    }

    #[test]
    fn get_my_rights_fail_no_jwt() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().get_my_rights("local");

        assert!(res.is_err());
    }

    #[test]
    fn get_strategies_ok() {
        let _m = mockito::mock("GET", "/strategies")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "getStrategies",
                    "volatile": {},
                    "result": [
                      "local",
                      "facebook"
                    ]
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().get_strategies();

        assert!(res.is_ok());
        let strategies = res.unwrap();
        assert_eq!(strategies.len(), 2);
    }

    #[test]
    fn get_strategies_fail_error() {
        let _m = mockito::mock("GET", "/strategies")
            .with_status(403)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 403,
                    "error": {
                      "message": "Forbidden action [null/null/auth/getStrategies] for user -1",
                      "status": 403,
                      "stack": "ForbiddenError: Forbidden action [null/null/auth/getStrategies] for user -1\n"
                    },
                    "controller": "auth",
                    "action": "getStrategies",
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().get_strategies();

        assert!(res.is_err());
    }

    #[test]
    fn login_ok() {
        let _m = mockito::mock("POST", "/_login/local")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "login",
                    "volatile": {},
                    "result": {
                      "_id": "user-kuid",
                      "jwt": "aSuperJwtToken",
                      "expiresAt": 1321085955000,
                      "ttl": 360000
                    }   
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().login("local", "admin", "password");

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), k.jwt());
    }

    #[test]
    fn login_fail_bad_credentials() {
        let _m = mockito::mock("POST", "/_login/local")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 401,
                    "error": {
                      "message": "wrong username or password",
                      "status": 401,
                      "stack": "UnauthorizedError: wrong username or password\n    at new UnauthorizedError (/var/app/node_modules/kuzzle-common-objects/lib/errors/unauthorizedError.js:5:5)\n    at authCB (/var/app/lib/api/core/auth/passportWrapper.js:71:25)\n    at allFailed (/var/app/node_modules/passport/lib/middleware/authenticate.js:107:18)\n    at attempt (/var/app/node_modules/passport/lib/middleware/authenticate.js:180:28)\n    at Strategy.strategy.fail (/var/app/node_modules/passport/lib/middleware/authenticate.js:297:9)\n    at verified (/var/app/plugins/available/kuzzle-plugin-auth-passport-local/node_modules/passport-local/lib/strategy.js:82:30)\n    at ret.then.then.result (/var/app/lib/api/core/plugins/pluginsManager.js:531:13)\n    at tryCatcher (/var/app/node_modules/bluebird/js/release/util.js:16:23)\n    at Promise._settlePromiseFromHandler (/var/app/node_modules/bluebird/js/release/promise.js:512:31)\n    at Promise._settlePromise (/var/app/node_modules/bluebird/js/release/promise.js:569:18)\n    at Promise._settlePromise0 (/var/app/node_modules/bluebird/js/release/promise.js:614:10)\n    at Promise._settlePromises (/var/app/node_modules/bluebird/js/release/promise.js:694:18)\n    at _drainQueueStep (/var/app/node_modules/bluebird/js/release/async.js:138:12)\n    at _drainQueue (/var/app/node_modules/bluebird/js/release/async.js:131:9)\n    at Async._drainQueues (/var/app/node_modules/bluebird/js/release/async.js:147:5)\n    at Immediate.Async.drainQueues (/var/app/node_modules/bluebird/js/release/async.js:17:14)\n    at runCallback (timers.js:810:20)\n    at tryOnImmediate (timers.js:768:5)\n    at processImmediate [as _immediateCallback] (timers.js:745:5)"
                    },
                    "controller": "auth",
                    "action": "login",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().login("local", "admin", "badPassword");

        assert!(res.is_err());
    }

    #[test]
    fn login_fail_empty_args() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().login("local", "", "");

        assert!(res.is_err());
    }

    #[test]
    fn logout_ok() {
        let _m = mockito::mock("POST", "/_logout")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "logout",
                    "volatile": {},
                    "result": {}   
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let res = k.auth().logout();

        assert!(res.is_ok());
        assert_eq!("", k.jwt());
    }

    #[test]
    fn logout_fail_no_jwt() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let res = k.auth().logout();

        assert!(res.is_err());
    }

    #[test]
    fn update_my_credentials_ok() {
        let _m = mockito::mock("PUT", "/credentials/local/_me/_update")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 200,
                    "error": null,
                    "controller": "auth",
                    "action": "updateMyCredentials",
                    "result": {
                      "username": "newUser",
                      "kuid": "myNewUserKUID"
                    }
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let mut new_content = HashMap::new();
        new_content.insert("password".to_string(), serde_json::to_value("newpassword").unwrap());
        let res = k.auth().update_my_credentials("local", &new_content);

        assert!(&res.is_ok());
        assert_eq!(
            res.unwrap().get("username").unwrap().as_str().unwrap(),
            "newUser"
        );
    }

    #[test]
    fn update_my_credentials_fail_error() {
        let _m = mockito::mock("PUT", "/credentials/local/_me/_update")
            .match_header("authorization", "Bearer mySuperToken")
            .with_status(412)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "requestId": "da9040aa-9529-4fb9-b627-a38736321364",
                    "status": 412,
                    "error": {
                      "message": "A strategy already exists for user \"-1\".",
                      "status": 412,
                      "stack": "Precondition Failed: A strategy already exists for user \"-1\".\n    at new PreconditionError (/var/app/node_modules/kuzzle-common-objects/lib/errors/preconditionError.js:5:5)\n    at exists.then.exists (/var/app/plugins/available/kuzzle-plugin-auth-passport-local/lib/index.js:224:17)\n    at tryCatcher (/var/app/node_modules/bluebird/js/release/util.js:16:23)\n    at Promise._settlePromiseFromHandler (/var/app/node_modules/bluebird/js/release/promise.js:512:31)\n    at Promise._settlePromise (/var/app/node_modules/bluebird/js/release/promise.js:569:18)\n    at Promise._settlePromise0 (/var/app/node_modules/bluebird/js/release/promise.js:614:10)\n    at Promise._settlePromises (/var/app/node_modules/bluebird/js/release/promise.js:694:18)\n    at _drainQueueStep (/var/app/node_modules/bluebird/js/release/async.js:138:12)\n    at _drainQueue (/var/app/node_modules/bluebird/js/release/async.js:131:9)\n    at Async._drainQueues (/var/app/node_modules/bluebird/js/release/async.js:147:5)\n    at Immediate.Async.drainQueues (/var/app/node_modules/bluebird/js/release/async.js:17:14)\n    at runCallback (timers.js:810:20)\n    at tryOnImmediate (timers.js:768:5)\n    at processImmediate [as _immediateCallback] (timers.js:745:5)"
                    },
                    "controller": "auth",
                    "action": "updateMyCredentials",
                    "volatile": {},
                    "result": null
                }"#,
            )
            .create();

        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let mut new_content = HashMap::new();
        new_content.insert("password".to_string(), serde_json::to_value("newpassword").unwrap());
        let res = k.auth().update_my_credentials("local", &new_content);

        assert!(res.is_err());
    }

    #[test]
    fn update_my_credentials_fail_empty_args() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        k.set_jwt("mySuperToken".to_string());
        let mut new_content = HashMap::new();
        new_content.insert("password".to_string(), serde_json::to_value("newpassword").unwrap());
        let res = k.auth().update_my_credentials("local", &new_content);

        assert!(res.is_err());
    }

    #[test]
    fn update_my_credentials_fail_not_logged_in() {
        let mut k = Kuzzle::new(Http::new(KuzzleOptions::new("localhost", 7512)));
        k.connect().expect("Unable to connect to Kuzzle server");
        let mut new_content = HashMap::new();
        new_content.insert("password".to_string(), serde_json::to_value("newpassword").unwrap());
        let res = k.auth().update_my_credentials("local", &new_content);

        assert!(res.is_err());
    }
}
