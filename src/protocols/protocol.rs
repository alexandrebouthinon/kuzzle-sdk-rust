use crate::types::{KuzzleRequest, KuzzleResponse, QueryOptions};
use std::error::Error;

pub trait Protocol {
    fn once(&self);
    fn listener_count(&self);
    fn connect(&self);
    fn send(&self, req: KuzzleRequest, options: QueryOptions) -> Result<KuzzleResponse, Box<Error>>;
    fn close(&self);
    fn state(&self);
    fn request_history(&self);
    fn start_queuing(&self);
    fn stop_queuing(&self);
    fn clear_queue(&self);
}
