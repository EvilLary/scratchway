use scratchway::wayland::*;
use scratchway::{log, Object};
use scratchway::connection::{Reader, Writer, WaylandBuffer};
use scratchway::events::{Message, WlEvent};
use super::xdg_shell::*;

scr_scanner::generate!("./protocols/wlr-layer-shell-unstable-v1.xml");
