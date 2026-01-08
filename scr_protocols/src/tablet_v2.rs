use scratchway::wayland::*;
use scratchway::connection::{Reader, WaylandBuffer, Writer};
use scratchway::events::*;
use scratchway::prelude::*;
use scratchway::log;

scr_scanner::generate!("./protocols/tablet-v2.xml");
