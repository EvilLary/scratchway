#![allow(unused)]

use crate::events::*;
use std::{cell::Cell, io::{Read, Write}, os::unix::net::UnixStream, rc::Rc};

#[derive(Debug)]
pub struct Connection {
    pub(crate) socket: UnixStream,
    pub(crate) id_counter: IdCounter,
    pub(crate) in_buffer: Vec<u8>,
    pub(crate) out_buffer: Vec<u8>,
}

impl Connection {
    pub fn connect() -> std::io::Result<Self> {
        let wayland_disp = std::env::var_os("WAYLAND_DISPLAY").unwrap_or("wayland-0".into());
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or("/tmp/".into());
        let socket = UnixStream::connect(std::path::PathBuf::from(runtime_dir).join(wayland_disp))?;
        Ok(Self {
            socket,
            id_counter: IdCounter::new(),
            in_buffer: vec![0; 4096],
            out_buffer: vec![0; 4096],
        })
    }

    pub fn display(&self) -> crate::core::wl_display::WlDisplay {
        let display_id = self.id_counter.get_new();
        crate::core::wl_display::WlDisplay::new(display_id)
    }

    pub(crate) fn new_id(&self) -> u32 {
        self.id_counter.get_new()
    }

    pub(crate) fn write_request<const S: usize>(&mut self, msg: Message<S>) {
        self.socket.write(msg.data());
    }

    pub fn blocking_read<'a>(&'a mut self) -> EventIter<'a> {
        let read = self.socket.read(&mut self.in_buffer).unwrap();
        EventIter::new(&self.in_buffer[..read])
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.socket.flush()
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
            current: Cell::new(0),
        }
    }
    pub(crate) const fn get_new(&self) -> u32 {
        let new = self.current.get() + 1;
        self.current.replace(new);
        new
    }
}
