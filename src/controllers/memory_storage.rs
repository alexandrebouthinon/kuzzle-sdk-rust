use crate::kuzzle::Kuzzle;

pub struct MemoryStorageController<'a>(pub &'a mut Kuzzle);

impl<'a> MemoryStorageController<'a> {
    fn _kuzzle(&'a mut self) -> &'a mut Kuzzle {
        self.0
    }
}
