use crate::protocols::{Protocol, State};
use crate::types::{KuzzleOptions, KuzzleRequest, KuzzleResponse, QueryOptions};
use std::error::Error;

pub struct Websocket {
    _options: KuzzleOptions,
    _state: State,
}

impl Websocket {
    pub fn new(options: KuzzleOptions) -> Websocket {
        Websocket {
            _options: options,
            _state: State::Offline,
        }
    }
}

impl Protocol for Websocket {
    fn once(&self) {
        unimplemented!();
    }

    fn listener_count(&self) {
        unimplemented!();
    }

    fn is_ready(&self) -> bool {
        match self._state {
            State::Ready => true,
            State::Offline => false,
        }
    }

    fn connect(&mut self) -> Result<(), Box<Error>> {
        unimplemented!();
    }

    fn send(
        &self,
        _req: KuzzleRequest,
        _options: QueryOptions,
    ) -> Result<KuzzleResponse, Box<Error>> {
        unimplemented!();
    }

    fn close(&mut self) {
        self._state = State::Offline;
    }
}
