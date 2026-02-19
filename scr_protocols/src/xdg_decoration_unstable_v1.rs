#![allow(unused_mut)]
use scratchway::wayland::*;
use scratchway::*;
use scratchway::wire::*;
use scratchway::log;

use crate::xdg_shell::*;

scr_scanner::generate!("./protocols/xdg-decoration-unstable-v1.xml");
