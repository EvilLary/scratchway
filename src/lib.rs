#![allow(internal_features)]

pub mod connection;
pub mod events;
pub mod wayland;

pub mod prelude {
    pub use crate::connection::{Connection, Object, State};
    pub use crate::events::WlEvent;
}

mod utils;
