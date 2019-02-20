use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct CollectionController<'a>(pub &'a mut Kuzzle);

impl<'a> CollectionController<'a> {
    pub fn create(&'a mut self, options: QueryOptions) {
        &self
            .kuzzle()
            .query(KuzzleRequest::new("collection", "create"), options);
    }

    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}
