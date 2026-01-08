use scratchway::wayland::*;
use scratchway::connection::{Reader, WaylandBuffer, Writer};
use scratchway::events::*;
use scratchway::prelude::*;
use scratchway::log;

use crate::xdg_shell::*;

scr_scanner::generate!("./protocols/wlr-layer-shell-unstable-v1.xml");
