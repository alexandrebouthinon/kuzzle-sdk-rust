use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct BulkController<'a>(pub &'a Kuzzle);

impl<'a> BulkController<'a> {
    pub fn import(&self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("bulk", "import");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
