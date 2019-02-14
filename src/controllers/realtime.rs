use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct RealtimeController<'a>(pub &'a Kuzzle);

impl<'a> RealtimeController<'a> {
    pub fn subscribe(&self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("realtime", "subscribe");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
