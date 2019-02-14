use crate::kuzzle::Kuzzle;

pub struct MemoryStorageController<'a>(pub &'a Kuzzle);

impl<'a> MemoryStorageController<'a> {
    fn _kuzzle(&self) -> &'a Kuzzle {
        &self.0
    }
}
