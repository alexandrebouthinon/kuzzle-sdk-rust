use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct RealtimeController<'a>(pub &'a mut Kuzzle);

impl<'a> RealtimeController<'a> {
    pub fn subscribe(&'a mut self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("realtime", "subscribe");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}
