#![allow(unused)]
#![feature(unix_socket_ancillary_data)]

use crate::events::*;
use std::{
    cell::Cell,
    io::{self, IoSlice, Read, Write},
    marker::PhantomData,
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
        unix::net::UnixStream,
    },
    rc::Rc,
};

use crate::utils::{Bucket, syscall};

pub static DEBUG: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| unsafe {
    let env = libc::getenv(c"WAYLAND_DEBUG".as_ptr()).cast_const();
    !env.is_null() && libc::strcmp(env, c"1".as_ptr().cast()) == 0
});

pub static TRACE: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| unsafe {
    let env = libc::getenv(c"SCR_TRACE".as_ptr()).cast_const();
    !env.is_null() && libc::strcmp(env, c"1".as_ptr().cast()) == 0
});

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

    pub(crate) reader: WaylandBuffer<Reader>,
    pub(crate) writer: WaylandBuffer<Writer>,
}

impl Connection {
    pub fn connect() -> std::io::Result<Self> {
        let wayland_disp = std::env::var_os("WAYLAND_DISPLAY").unwrap_or("wayland-0".into());
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or("/tmp/".into());
        let socket = UnixStream::connect(std::path::PathBuf::from(runtime_dir).join(wayland_disp))?;
        let debug = unsafe { !libc::getenv(c"WAYLAND_DEBUG".as_ptr()).is_null() };
        if *TRACE {
            eprintln!(
                "[\x1b[36mTRACE\x1b[0m]: connected to wayland socket at {:?}",
                socket.peer_addr().unwrap()
            );
        }
        Ok(Self {
            socket,
            id_counter: IdCounter::new(),
            in_buffer: Bucket::full(),
            out_buffer: Bucket::new(),
            reader: WaylandBuffer::<Reader>::new(), // Thanks Rust
            writer: WaylandBuffer::<Writer>::new(),
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

    pub(crate) fn write_request<const S: usize>(&self, msg: Message<S>) {
        // TODO: check buffer len to flush
        let me = self.get_mut();
        if !self.writer.data.can_fit(msg.data().len()) {
            me.writer.flush(self.display_fd());
        }
        me.writer.write_msg(msg);
    }

    fn get_mut(&self) -> &mut Self {
        // SAFETY: yum yum
        trust_me_bro! {
            (self as *const Self as *mut Self)
                .as_mut()
                .unwrap_unchecked()
        }
    }

    pub fn dispatch_events<S: State>(&mut self, state: &mut S) -> io::Result<()> {
        let read = self.read_events()?;
        let events = EventIter::new(&self.in_buffer.as_slice()[..read]);
        for event in events {
            state.handle_event(self, event);
        }
        Ok(())
    }

    fn read_events(&mut self) -> io::Result<usize> {
        self.writer.flush(self.display_fd());
        let read = unsafe {
            libc::recv(
                self.display_fd(),
                self.in_buffer.as_mut_ptr().cast(),
                self.in_buffer.len(),
                libc::MSG_NOSIGNAL,
            )
        };
        if read <= 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(read as usize)
        }
    }

    pub fn blocking_read<'a>(&'a self) -> EventIter<'a> {
        let conn = self.get_mut();
        let _ = conn.writer.flush(self.display_fd());
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

    // TODO: Remove this
    pub fn flush(&self) -> std::io::Result<()> {
        self.get_mut().writer.flush(self.display_fd())
    }

    pub fn add_fd(&self, fd: RawFd) {
        if *TRACE {
            eprintln!("[\x1b[36mTRACE\x1b[0m]: Add fd {} to pool", fd,);
        }
        self.get_mut()
            .writer
            .fds
            .push(unsafe { OwnedFd::from_raw_fd(fd) });
    }

    pub fn roundtrip(&mut self, state: &mut impl State) -> std::io::Result<()> {
        let display = self.display();
        let wl_callback = display.sync(self);
        let read = self.read_events()?;
        let events = EventIter::new(&self.in_buffer.as_slice()[..read]);
        for event in events {
            if event.header.id == wl_callback.id {
                let _ = wl_callback.parse_event(event); // just for debugs
                break;
            }
            state.handle_event(self, event);
        }
        // self.dispatch_events(state)?;
        Ok(())
        // todo!()
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

#[derive(Debug)]
pub struct Reader;
#[derive(Debug)]
pub struct Writer;

#[derive(Debug)]
pub(crate) struct WaylandBuffer<T> {
    pub(crate) data: Bucket<u8, MAX_BUFFER_SIZE>,
    pub(crate) fds:  Bucket<OwnedFd, 8>,
    _ghost:          PhantomData<T>,
}

impl WaylandBuffer<Reader> {
    pub fn new() -> WaylandBuffer<Reader> {
        Self {
            data:   Bucket::full(),
            fds:    Bucket::new(),
            _ghost: PhantomData,
        }
    }

    pub fn get_fd(&mut self) -> Option<OwnedFd> {
        self.fds.pop()
    }

    pub fn event_iter(&self, size: usize) -> EventIter<'_> {
        EventIter::new(&self.data[..size])
    }
}

impl WaylandBuffer<Writer> {
    pub fn new() -> WaylandBuffer<Writer> {
        Self {
            data:   Bucket::new(),
            fds:    Bucket::new(),
            _ghost: PhantomData::<Writer>,
        }
    }

    pub fn write_msg<const S: usize>(&mut self, msg: Message<S>) {
        self.data.extend_from_slice(msg.data());
    }

    pub fn add_fd(&mut self, fd: RawFd) {
        self.fds.push(unsafe { OwnedFd::from_raw_fd(fd) });
    }

    pub fn flush(&mut self, display_fd: RawFd) -> std::io::Result<()> {
        if self.data.empty() {
            return Ok(());
        }
        if self.fds.empty() {
            let ret = unsafe {
                syscall!(libc::send(
                    display_fd,
                    self.data.as_ptr().cast(),
                    self.data.len(),
                    libc::MSG_NOSIGNAL,
                ))?
            };
            if *TRACE {
                eprintln!(
                    "[\x1b[36mTRACE\x1b[0m]: Written {} bytes to fd {} out of {}",
                    ret,
                    display_fd,
                    self.data.len(),
                );
            }
            debug_assert_eq!(self.data.len(), ret as usize); // ??
            unsafe {
                self.data.set_len(self.data.len() - (ret as usize));
            }
        } else {
            unsafe {
                let mut buf = [0u8; 56];
                let fds_bytes = self.fds.as_bytes();
                let fds_len = fds_bytes.len();
                let required_len = libc::CMSG_SPACE(fds_len as u32);
                let buf = &mut buf[..required_len as usize];

                let mut io = libc::iovec {
                    iov_base: self.data.as_mut_ptr().cast(),
                    iov_len:  self.data.len(),
                };
                let mut msghdr = libc::msghdr {
                    msg_iov:        &mut io as *mut _,
                    msg_iovlen:     1,
                    msg_control:    buf.as_mut_ptr().cast(),
                    msg_controllen: buf.len(),
                    msg_name:       core::ptr::null_mut(),
                    msg_namelen:    0,
                    msg_flags:      0,
                };
                let mut cmsghdr = libc::CMSG_FIRSTHDR(&msghdr as *const _);

                // ?? I'm not sure if this exactly correct
                (*cmsghdr).cmsg_len = libc::CMSG_LEN(fds_len as u32) as usize;
                (*cmsghdr).cmsg_level = libc::SOL_SOCKET;
                (*cmsghdr).cmsg_type = libc::SCM_RIGHTS;

                let data = libc::CMSG_DATA(cmsghdr);

                if *TRACE {
                    eprintln!(
                        "[\x1b[36mTRACE\x1b[0m]: Sending {:?} fd(s), msghdr: {:?}",
                        self.fds.len(),
                        msghdr
                    );
                }

                // we've only got 10 fds with 4B each it's 40B, CMSG_SPACE(40) = 56
                libc::memcpy(data.cast(), fds_bytes.as_ptr().cast(), fds_len);
                msghdr.msg_controllen = libc::CMSG_SPACE(fds_len as u32) as usize;

                let len = syscall!(libc::sendmsg(display_fd, &msghdr as *const libc::msghdr, 0))?;
                debug_assert_eq!(self.data.len(), len as usize); // ?
                unsafe {
                    self.data.set_len(self.data.len() - (len as usize));
                }
                self.fds.clear();
            }
        }
        Ok(())
    }
}
