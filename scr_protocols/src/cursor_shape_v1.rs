#![allow(unused_mut, unused)]
use scratchway::wayland::*;
use scratchway::connection::{Reader, WaylandBuffer, Writer};
use scratchway::events::*;
use scratchway::prelude::*;
use scratchway::log;
use crate::tablet_v2::*;

scr_scanner::generate!("./protocols/cursor-shape-v1.xml");
