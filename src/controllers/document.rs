use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct DocumentController<'a>(pub &'a mut Kuzzle);

impl<'a> DocumentController<'a> {
    pub fn create(&'a mut self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("document", "create");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}
