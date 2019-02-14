use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct ServerController<'a>(pub &'a Kuzzle);

impl<'a> ServerController<'a> {
    pub fn now(&self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("server", "now");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
