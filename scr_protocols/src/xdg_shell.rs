#![allow(unused_mut, unused)]

use scratchway::wayland::*;
use scratchway::connection::{Reader, WaylandBuffer, Writer};
use scratchway::events::*;
use scratchway::prelude::*;
use scratchway::log;

scr_scanner::generate!("./protocols/xdg-shell.xml");
