#![allow(unused)]
#![feature(unix_socket_ancillary_data)]

use crate::events::*;
use std::{
    cell::Cell,
    io::{IoSlice, Read, Write},
    os::{
        fd::{AsRawFd, RawFd},
        unix::net::UnixStream,
    },
    rc::Rc,
};

use crate::utils::Bucket;

static DEBUG: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| unsafe { !libc::getenv(c"WAYLAND_DEBUG".as_ptr()).is_null() });

macro_rules! trust_me_bro {
    ($cooked:expr) => {
        unsafe { $cooked }
    };
}

const MAX_BUFFER_SIZE: usize = 4096;
#[derive(Debug)]
pub struct Connection {
    pub(crate) socket:     UnixStream,
    pub(crate) id_counter: IdCounter,

    pub(crate) in_buffer:  Bucket<u8, MAX_BUFFER_SIZE>,
    pub(crate) out_buffer: Bucket<u8, MAX_BUFFER_SIZE>,
    pub(crate) out_fds:    [RawFd; 10],
}

impl Connection {
    pub fn connect() -> std::io::Result<Self> {
        let wayland_disp = std::env::var_os("WAYLAND_DISPLAY").unwrap_or("wayland-0".into());
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or("/tmp/".into());
        let socket = UnixStream::connect(std::path::PathBuf::from(runtime_dir).join(wayland_disp))?;
        let debug = unsafe { !libc::getenv(c"WAYLAND_DEBUG".as_ptr()).is_null() };
        let out_buffer = Bucket::new();
        let in_buffer = Bucket::full();
        Ok(Self {
            socket,
            id_counter: IdCounter::new(),
            in_buffer,
            out_buffer,
            out_fds: [0; 10],
        })
    }

    pub fn display_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }

    pub fn display(&self) -> crate::protocols::core::WlDisplay {
        crate::protocols::Object::from_id(1)
    }

    pub(crate) fn new_id(&self) -> u32 {
        self.id_counter.get_new()
    }

    pub(crate) fn write_request(&self, msg: &[u8]) {
        // TODO: check buffer len to flush
        if !self.out_buffer.can_fit(msg.len()) {
            self.flush();
        }
        self.get_mut().out_buffer.extend_from_slice(msg);
        // self.get_mut().socket.write(msg);
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
        // FIXME
        let read = unsafe {
            libc::recv(
                conn.display_fd(),
                conn.in_buffer.as_mut_ptr().cast(),
                conn.in_buffer.len(),
                libc::MSG_NOSIGNAL,
            )
        };
        // FIXME
        if read <= 0 {
            panic!("Failed to read from wayland socket");
        }
        EventIter::new(&self.in_buffer.as_slice()[..read as usize])
    }

    pub(crate) fn add_fd(&self, fd: RawFd) {}

    fn fflush(&self) -> std::io::Result<()> {
        // let iov = IoSlice::new(&self.out_buffer);
        self.get_mut().socket.flush()
    }

    pub fn flush(&self) -> std::io::Result<()> {
        let me = self.get_mut();
        // FIXME
        unsafe {
            assert!(
                libc::send(
                    me.display_fd(),
                    me.out_buffer.as_ptr().cast(),
                    me.out_buffer.len(),
                    libc::MSG_NOSIGNAL
                ) != -1
            );
            me.out_buffer.clear();
        }
        Ok(())
    }

    pub fn roundtrip(&self, state: &mut impl State) -> std::io::Result<()> {
        let display = self.display();
        let wl_callback = display.sync(self);
        self.flush();
        todo!()
    }
}

pub trait State {
    fn handle_event(&mut self, conn: &Connection, event: Event<'_>);
}

#[derive(Debug)]
pub(crate) struct IdCounter {
    pub(crate) current: Cell<u32>,
}

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
