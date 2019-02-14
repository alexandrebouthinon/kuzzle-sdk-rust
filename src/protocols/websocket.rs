use crate::protocols::Protocol;
use crate::types::{KuzzleOptions, KuzzleRequest, KuzzleResponse, QueryOptions};
use std::error::Error;

pub struct Websocket {
    _options: KuzzleOptions,
}

impl Websocket {
    pub fn new(options: KuzzleOptions) -> Websocket {
        Websocket { _options: options }
    }
}

impl Protocol for Websocket {
    fn once(&self) {
        unimplemented!();
    }

    fn listener_count(&self) {
        unimplemented!();
    }

    fn connect(&self) {
        unimplemented!();
    }

    fn send(
        &self,
        _req: KuzzleRequest,
        _options: QueryOptions,
    ) -> Result<KuzzleResponse, Box<Error>> {
        unimplemented!();
    }

    fn close(&self) {
        unimplemented!();
    }

    fn state(&self) {
        unimplemented!();
    }

    fn request_history(&self) {
        unimplemented!();
    }

    fn start_queuing(&self) {
        unimplemented!();
    }

    fn stop_queuing(&self) {
        unimplemented!();
    }

    fn clear_queue(&self) {
        unimplemented!();
    }
}
