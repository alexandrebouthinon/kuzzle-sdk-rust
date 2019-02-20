use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct AuthController<'a>(pub &'a mut Kuzzle);

impl<'a> AuthController<'a> {
    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }

    pub fn login(&'a mut self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("auth", "login");
        self.kuzzle().query(req, options).is_ok();
    }
}
