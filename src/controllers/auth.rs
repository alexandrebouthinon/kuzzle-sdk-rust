use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct AuthController<'a>(pub &'a Kuzzle);

impl<'a> AuthController<'a> {
    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }

    pub fn login(&self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("auth", "login");
        self.kuzzle().query(req, options).is_ok();
    }
}
