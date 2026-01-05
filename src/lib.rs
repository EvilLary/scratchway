#![allow(internal_features)]

pub mod connection;
pub mod events;
pub mod wayland;

pub mod prelude {
    pub use crate::connection::{Connection, Object, State};
    pub use crate::events::WlEvent;
}

pub use connection::{State, Object, Connection};
pub use events::WlEvent;

mod utils;
