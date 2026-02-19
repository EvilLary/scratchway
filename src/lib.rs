#![allow(internal_features)]

pub mod wayland;

pub mod connection;
mod objects;

pub use connection::wire;
pub use connection::{Connection, Global};
pub use objects::{Listener, WlInterface, WlProxy, Interface};

mod utils;
