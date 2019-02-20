use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct BulkController<'a>(pub &'a mut Kuzzle);

impl<'a> BulkController<'a> {
    pub fn import(&'a mut self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("bulk", "import");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}
