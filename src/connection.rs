use crate::events::*;
use crate::log;
use crate::wayland::wl_display;
use std::sync::RwLock;
use std::{
    cell::Cell,
    io,
    marker::PhantomData,
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
        unix::net::UnixStream,
    },
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

static IDCOUNTER: IdCounter = IdCounter::new();

#[derive(Debug)]
pub struct Connection {
    pub(crate) socket: UnixStream,
    pub(crate) reader: WaylandBuffer<Reader>,
    pub(crate) writer: WaylandBuffer<Writer>,
}

impl Connection {
    pub fn connect() -> std::io::Result<Self> {
        let wayland_disp = std::env::var_os("WAYLAND_DISPLAY").unwrap_or("wayland-0".into());
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or("/tmp/".into());
        let socket = UnixStream::connect(std::path::PathBuf::from(runtime_dir).join(wayland_disp))?;
        log!(
            TRACE,
            "connected to wayland socket at {:?}",
            socket.peer_addr().unwrap()
        );
        Ok(Self {
            reader: WaylandBuffer::<Reader>::new(socket.as_raw_fd()), // Thanks Rust
            writer: WaylandBuffer::<Writer>::new(socket.as_raw_fd()),
            socket,
        })
    }

    pub fn display_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }

    pub fn display(&self) -> wl_display::WlDisplay {
        Object::from_id(1)
    }

    pub fn dispatch_events<S: State>(&self, state: &mut S) -> io::Result<()> {
        let read = self.read_events()?;
        let data = self.reader.data.read().unwrap();
        let events = EventIter::new(&data[..read]);
        for event in events {
            state.handle_event(self, event);
        }
        Ok(())
    }

    fn read_events(&self) -> io::Result<usize> {
        self.writer.send()?;
        self.reader.recv()
    }

    pub fn roundtrip(&self, state: &mut impl State) -> std::io::Result<()> {
        let display = self.display();
        let wl_callback = display.sync(&self.writer);
        let read = self.read_events()?;
        let data = self.reader.data.read().unwrap();
        let events = EventIter::new(&data[..read]);
        for event in events {
            if wl_callback.id() == event.header.id {
                wl_callback.parse_event(&self.reader, event); // just for debugs
                break;
            }
            state.handle_event(self, event);
        }
        Ok(())
    }

    #[inline(always)]
    pub fn writer(&self) -> &WaylandBuffer<Writer> {
        &self.writer
    }

    #[inline(always)]
    pub fn reader(&self) -> &WaylandBuffer<Reader> {
        &self.reader
    }

    pub fn flush(&self) -> std::io::Result<()> {
        self.writer().send()
    }
}

pub trait State {
    fn handle_event(&mut self, conn: &Connection, event: WlEvent<'_>);
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

unsafe impl Sync for IdCounter {}

#[derive(Debug)]
pub struct Reader;
#[derive(Debug)]
pub struct Writer;

const MAX_BUFFER_SIZE: usize = 4096;

//  TODO: find better way to handle interior mutability
#[derive(Debug)]
pub struct WaylandBuffer<T> {
    pub(crate) data: RwLock<Bucket<u8, MAX_BUFFER_SIZE>>,
    pub(crate) fds: RwLock<Bucket<OwnedFd, 8>>,
    pub(crate) display_fd: RawFd,
    _ghost: PhantomData<T>,
}

impl WaylandBuffer<Reader> {
    fn new(display_fd: RawFd) -> WaylandBuffer<Reader> {
        Self {
            data: RwLock::new(Bucket::full()),
            fds: RwLock::new(Bucket::new()),
            display_fd,
            _ghost: PhantomData,
        }
    }

    pub fn get_fd(&self) -> Option<OwnedFd> {
        self.fds.write().unwrap().pop()
    }

    fn recv(&self) -> std::io::Result<usize> {
        let mut buf = [0u8; 56];
        let mut fds = self.fds.write().unwrap();
        fds.clear();
        let mut data = self.data.write().unwrap();
        unsafe {
            let mut msg_name: libc::sockaddr_un = core::mem::zeroed();
            let mut msghdr: libc::msghdr = core::mem::zeroed();

            msghdr.msg_name = (&raw mut msg_name).cast();
            msghdr.msg_namelen = size_of::<libc::sockaddr_un>() as u32;

            let mut iov = libc::iovec {
                iov_base: data.as_mut_ptr().cast(),
                iov_len: data.len(),
            };

            msghdr.msg_iov = (&raw mut iov).cast();
            msghdr.msg_iovlen = 1;

            msghdr.msg_controllen = buf.len();
            msghdr.msg_control = buf.as_mut_ptr().cast();

            let len = syscall!(libc::recvmsg(
                self.display_fd,
                &raw mut msghdr,
                libc::MSG_CMSG_CLOEXEC
            ))?;

            if msghdr.msg_controllen > 0 {
                // lol this is probably not correct, works tho
                let cmsghdr = *libc::CMSG_FIRSTHDR(&raw const msghdr);
                if cmsghdr.cmsg_type == libc::SCM_RIGHTS {
                    let len = cmsghdr.cmsg_len;
                    let data = &buf[size_of::<libc::cmsghdr>()..len];
                    let raw_fds = core::slice::from_raw_parts(
                        data.as_ptr() as *const i32,
                        data.len() / size_of::<i32>(),
                    );
                    for fd in raw_fds {
                        fds.push(OwnedFd::from_raw_fd(*fd));
                    }
                    log!(TRACE, "Recived ancillay data: {:?}", cmsghdr);
                }
            }
            log!(TRACE, "Recieved {} bytes from fd {}", len, self.display_fd);
            Ok(len as usize)
        }
    }
}

impl WaylandBuffer<Writer> {
    fn new(display_fd: RawFd) -> WaylandBuffer<Writer> {
        Self {
            data: RwLock::new(Bucket::new()),
            fds: RwLock::new(Bucket::new()),
            display_fd,
            _ghost: PhantomData::<Writer>,
        }
    }

    pub fn new_id(&self) -> u32 {
        IDCOUNTER.get_new()
    }

    pub fn write_request(&self, msg: &[u8]) {
        if !self.data.read().unwrap().can_fit(msg.len()) {
            log!(TRACE, "Buffer can't fit additional {} bytes", msg.len());
            self.send().unwrap();
        }
        self.data.write().unwrap().extend_from_slice(msg);
    }

    pub fn add_fd(&self, fd: RawFd) {
        self.fds
            .write()
            .unwrap()
            .push(unsafe { OwnedFd::from_raw_fd(fd) });
        log!(TRACE, "Added fd {} to pool", fd,);
    }

    fn send(&self) -> std::io::Result<()> {
        let mut data = self.data.write().unwrap();
        let mut fds = self.fds.write().unwrap();
        if data.empty() {
            return Ok(());
        }

        let flags = libc::MSG_NOSIGNAL;
        let len = if fds.empty() {
            let len = unsafe {
                syscall!(libc::send(
                    self.display_fd,
                    data.as_ptr().cast(),
                    data.len(),
                    flags,
                ))?
            };
            len as usize
        } else {
            unsafe {
                // we've only got 10 fds with 4B each it's 40B, CMSG_SPACE(40) = 56
                let mut buf = [0u8; 56];
                let fds_bytes = fds.as_bytes();
                let fds_len = fds_bytes.len();
                let required_len = libc::CMSG_SPACE(fds_len as u32) as usize;
                let buf = &mut buf[..required_len];

                let mut io = libc::iovec {
                    iov_base: data.as_mut_ptr().cast(),
                    iov_len: data.len(),
                };

                let msghdr = libc::msghdr {
                    msg_iov: &raw mut io,
                    msg_iovlen: 1,
                    msg_control: buf.as_mut_ptr().cast(),
                    msg_controllen: required_len,
                    msg_name: core::ptr::null_mut(),
                    msg_namelen: 0,
                    msg_flags: 0,
                };

                let cmsghdr = libc::CMSG_FIRSTHDR(&raw const msghdr);
                // ?? I'm not sure if this exactly correct
                (*cmsghdr).cmsg_len = libc::CMSG_LEN(fds_len as u32) as usize;
                (*cmsghdr).cmsg_level = libc::SOL_SOCKET;
                (*cmsghdr).cmsg_type = libc::SCM_RIGHTS;

                let data = libc::CMSG_DATA(cmsghdr);
                libc::memcpy(data.cast(), fds_bytes.as_ptr().cast(), fds_len);

                let len = syscall!(libc::sendmsg(self.display_fd, &raw const msghdr, flags))?;
                fds.clear();
                len as usize
            }
        };

        let data_len = data.len();
        log!(
            TRACE,
            "Written {} bytes to fd {} out of {}",
            len,
            self.display_fd,
            data_len,
        );
        debug_assert_eq!(data_len, len as usize); // ??
        unsafe {
            data.set_len(data_len - (len as usize));
        }
        Ok(())
    }
}

pub trait Object {
    type Event<'a>;
    fn from_id(id: u32) -> Self;

    fn id(&self) -> u32;

    fn interface(&self) -> &'static str;

    fn parse_event<'a>(
        &self, reader: &WaylandBuffer<Reader>, event: crate::events::WlEvent<'a>,
    ) -> Self::Event<'a>;
}
