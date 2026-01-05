use scratchway::wayland::*;
use scratchway::{log, Object};
use scratchway::connection::{Reader, Writer, WaylandBuffer};
use scratchway::events::{Message, WlEvent};

scr_scanner::generate!("./protocols/single-pixel-buffer-v1.xml");
