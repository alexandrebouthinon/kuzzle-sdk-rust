use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct DocumentController<'a>(pub &'a Kuzzle);

impl<'a> DocumentController<'a> {
    pub fn create(&self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("document", "create");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
