#![allow(unused)]

use crate::events::*;
use std::{
    cell::Cell,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, RawFd},
        unix::net::UnixStream,
    },
    rc::Rc,
};

static DEBUG: bool = false;

macro_rules! trust_me_bro {
    ($cooked:expr) => {
        unsafe { $cooked }
    };
}

#[derive(Debug)]
pub struct Connection {
    pub(crate) socket: UnixStream,
    pub(crate) id_counter: IdCounter,
    pub(crate) in_buffer: Vec<u8>,
    pub(crate) out_buffer: Vec<u8>,
    pub(crate) debug: bool,
}

impl Connection {
    pub fn connect() -> std::io::Result<Self> {
        let wayland_disp = std::env::var_os("WAYLAND_DISPLAY").unwrap_or("wayland-0".into());
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or("/tmp/".into());
        let socket = UnixStream::connect(std::path::PathBuf::from(runtime_dir).join(wayland_disp))?;
        let debug = unsafe { !libc::getenv(c"WAYLAND_DEBUG".as_ptr()).is_null() };
        Ok(Self {
            socket,
            id_counter: IdCounter::new(),
            in_buffer: vec![0; 4096],
            out_buffer: vec![0; 4096],
            debug,
        })
    }

    pub fn display_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }

    pub fn display(&self) -> crate::core::WlDisplay {
        crate::core::Object::from_id(1)
    }

    pub(crate) fn new_id(&self) -> u32 {
        self.id_counter.get_new()
    }

    pub(crate) fn write_request(&self, msg: &[u8]) {
        self.get_mut().socket.write(msg);
    }

    fn get_mut(&self) -> &mut Self {
        // SAFETY: trust me bro
        trust_me_bro! {
            (self as *const Self as *mut Self)
                .as_mut()
                .unwrap_unchecked()
        }
    }
    pub fn blocking_read<'a>(&'a self) -> EventIter<'a> {
        let _ = self.flush();
        let conn = self.get_mut();
        let read = conn.socket.read(&mut conn.in_buffer).unwrap();
        EventIter::new(&self.in_buffer[..read])
    }

    pub fn flush(&self) -> std::io::Result<()> {
        self.get_mut().socket.flush()
    }
}

impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.socket.read(buf)
    }
}

#[derive(Debug)]
pub(crate) struct IdCounter {
    pub(crate) current: Cell<u32>,
}

unsafe impl Sync for IdCounter {}
unsafe impl Send for IdCounter {}

impl IdCounter {
    pub(crate) const fn new() -> Self {
        Self {
            current: Cell::new(1),
        }
    }
    pub(crate) const fn get_new(&self) -> u32 {
        let new = self.current.get() + 1;
        self.current.replace(new);
        new
    }
}
