use scratchway::wayland::*;
use scratchway::{log, Object};
use scratchway::connection::{Reader, Writer, WaylandBuffer};
use scratchway::events::{Message, WlEvent};

scr_scanner::generate!("./protocols/xdg-shell.xml");
