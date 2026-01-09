#![allow(unused_mut, unused)]
use scratchway::wayland::*;
use scratchway::connection::{Reader, WaylandBuffer, Writer};
use scratchway::events::*;
use scratchway::prelude::*;
use scratchway::log;
use super::xdg_shell::*;

scr_scanner::generate!("./protocols/xdg-decoration-unstable-v1.xml");
