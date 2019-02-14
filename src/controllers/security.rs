use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct SecurityController<'a>(pub &'a Kuzzle);

impl<'a> SecurityController<'a> {
    pub fn create_credentials(&self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("security", "createCredentials");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
