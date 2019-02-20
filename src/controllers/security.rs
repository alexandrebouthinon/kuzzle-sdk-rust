use crate::kuzzle::Kuzzle;
use crate::types::{KuzzleRequest, QueryOptions};

pub struct SecurityController<'a>(pub &'a mut Kuzzle);

impl<'a> SecurityController<'a> {
    pub fn create_credentials(&'a mut self, options: QueryOptions) {
        let req: KuzzleRequest = KuzzleRequest::new("security", "createCredentials");
        self.kuzzle().query(req, options).is_ok();
    }

    fn kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}
