//! # kuzzle_sdk
//!
//! kuzzle_sdk is a Kuzzle SDK Rust implementation.
//!
//! It provides functions and methods to interact with a Kuzzle server or cluster
//!
//! If you're just starting with Kuzzle, check the product [website](https://kuzzle.io)
//! and [documentation](https://docs.kuzzle.io)

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod controllers;
pub mod event_emitter;
pub mod kuzzle;
pub mod protocols;
pub mod types;
