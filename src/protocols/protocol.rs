use crate::types::{KuzzleRequest, KuzzleResponse, QueryOptions};
use std::error::Error;

pub trait Protocol {
    fn once(&self);
    fn listener_count(&self);
    fn connect(&mut self) -> Result<(), Box<Error>>;
    fn send(&self, req: KuzzleRequest, options: QueryOptions)
        -> Result<KuzzleResponse, Box<Error>>;
    fn close(&mut self);
    fn is_ready(&self) -> bool;
}
