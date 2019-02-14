use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct CollectionController<'a>(pub &'a Kuzzle);

impl<'a> CollectionController<'a> {
    pub fn create(&self, options: QueryOptions) {
        &self
            .kuzzle()
            .query(KuzzleRequest::new("collection", "create"), options);
    }

    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
