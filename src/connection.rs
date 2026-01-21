use crate::events::*;
use crate::log;
use crate::wayland::wl_display;
use std::collections::HashMap;
use std::collections::VecDeque;
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

#[derive(Debug)]
pub struct Connection<S> {
    socket: UnixStream,

    reader: WaylandBuffer<Reader>,
    writer: WaylandBuffer<Writer>,

    event_queue: VecDeque<WlEvent>,
    object_mgr: ObjectManager<S>,
}

// FIXME FIXME FIXME FIXME FIXME FIXME
// Fix this mess
impl<S> Connection<S> {
    pub const WL_DISPLAY_ID: u32 = 1;

    pub fn connect() -> std::io::Result<Self> {
        let wayland_disp =
            std::env::var_os("WAYLAND_DISPLAY").ok_or(std::io::Error::other("WAYLAND_DISPLAY isn't set"))?;

        let runtime_dir =
            std::env::var_os("XDG_RUNTIME_DIR").ok_or(std::io::Error::other("XDG_RUNTIME_DIR isn't set"))?;

        let socket = UnixStream::connect(std::path::PathBuf::from(runtime_dir).join(wayland_disp))?;

        let mut object_mgr = ObjectManager {
            objects: HashMap::new(),
            dead_ids: Vec::new(),
            id_counter: IdCounter::new(),
        };
        object_mgr.objects.insert(Self::WL_DISPLAY_ID, None);

        log!(
            TRACE,
            "connected to wayland socket at {:?}",
            socket.peer_addr().unwrap()
        );

        Ok(Self {
            reader: WaylandBuffer::<Reader>::new(socket.as_raw_fd()), // Thanks Rust
            writer: WaylandBuffer::<Writer>::new(socket.as_raw_fd()),
            socket,
            event_queue: VecDeque::new(),
            object_mgr,
        })
    }

    pub fn display_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }

    pub fn display(&self) -> wl_display::WlDisplay {
        Object::from_id(Self::WL_DISPLAY_ID)
    }

    pub fn dispatch_events(&mut self, state: &mut S) {
        while let Some(event) = self.event_queue.pop_front() {
            self.send_event(state, event);
        }
    }

    fn send_event(&mut self, state: &mut S, event: WlEvent) {
        if event.header.id == Self::WL_DISPLAY_ID {
            let _ = self.handle_wldisplay(&event);
        }

        if let Some(cb) = self.object_mgr.get_callback(event.header.id) {
            cb(state, self, event);
        } else {
            if event.header.id == 1 {
                println!("DISPLAY_OP: {} - {}", event.header.opcode, event.data.escape_ascii());
            }
            log!(WAYLAND, "discarded event for #{}", event.header.id);
        }
    }

    fn read_events(&mut self) -> io::Result<()> {
        let len = self.reader.recv()?;
        let data = &self.reader.data;
        self.event_queue.extend(EventIter::new(&data[..len]));
        Ok(())
    }

    pub fn blocking_dispatch(&mut self, state: &mut S) -> std::io::Result<()> {
        self.flush()?;
        self.read_events()?;
        self.dispatch_events(state);
        Ok(())
    }

    pub fn roundtrip(&mut self, state: &mut S) -> std::io::Result<()> {
        self.dispatch_events(state);

        let display = self.display();
        let wl_callback = display.sync(self);

        self.flush()?;

        self.read_events()?;

        while let Some(event) = self.event_queue.pop_front() {
            if wl_callback.id() == event.header.id {
                wl_callback.parse_event(self, &event);
                break;
            }
            self.send_event(state, event);
        }

        Ok(())
    }

    fn handle_wldisplay(&mut self, event: &WlEvent) -> std::io::Result<()> {
        let parser = event.parser();
        match event.header.opcode {
            0 => {}, // TODO
            1 => {
                let id = parser.get_u32();
                self.object_mgr.remove_id(id)
            },
            _ => unreachable!(),
        }
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.send()
    }

    pub fn add_callback(&mut self, object: &impl Object, cb: CallbackFn<S>) {
        self.object_mgr.set_callback(object.id(), cb);
    }

    pub fn remove_id(&mut self, id: u32) {
        self.object_mgr.remove_id(id);
    }

    pub fn remove_callback(&mut self, id: u32) {
        self.object_mgr.remove_callback(id);
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn writer(&mut self) -> &mut WaylandBuffer<Writer> {
        &mut self.writer
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn reader(&mut self) -> &mut WaylandBuffer<Reader> {
        &mut self.reader
    }

    #[doc(hidden)]
    pub fn new_id(&mut self) -> u32 {
        self.object_mgr.new_id()
    }
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
const MAX_FD_LEN: usize = 12;

#[derive(Debug)]
pub struct WaylandBuffer<T> {
    pub(crate) data: Bucket<u8, MAX_BUFFER_SIZE>,
    // FIXME: Use VecDeque
    pub(crate) fds: VecDeque<OwnedFd>,
    // pub(crate) fds: Bucket<OwnedFd, MAX_FD_LEN>,
    pub(crate) display_fd: RawFd,
    _ghost: PhantomData<T>,
}

impl<T> WaylandBuffer<T> {
    const ALIGNED_LEN: usize = unsafe { libc::CMSG_SPACE((MAX_FD_LEN * size_of::<u32>()) as u32) as usize };
}

impl WaylandBuffer<Reader> {
    fn new(display_fd: RawFd) -> WaylandBuffer<Reader> {
        Self {
            data: Bucket::full(),
            fds: VecDeque::with_capacity(MAX_FD_LEN),
            // fds: Bucket::new(),
            display_fd,
            _ghost: PhantomData,
        }
    }

    pub fn get_fd(&mut self) -> Option<OwnedFd> {
        self.fds.pop_front()
    }

    fn recv(&mut self) -> std::io::Result<usize> {
        let mut ancillary_buf = [0u8; Self::ALIGNED_LEN];

        self.fds.clear();
        unsafe {
            let mut msghdr: libc::msghdr = core::mem::zeroed();

            let mut msg_name: libc::sockaddr_un = core::mem::zeroed();
            msghdr.msg_name = (&raw mut msg_name).cast();
            msghdr.msg_namelen = size_of::<libc::sockaddr_un>() as u32;

            let mut iov = libc::iovec {
                iov_base: self.data.as_mut_ptr().cast(),
                iov_len: self.data.len(),
            };
            msghdr.msg_iov = (&raw mut iov).cast();
            msghdr.msg_iovlen = 1;

            msghdr.msg_controllen = ancillary_buf.len();
            msghdr.msg_control = ancillary_buf.as_mut_ptr().cast();

            let len = syscall!(libc::recvmsg(
                self.display_fd,
                &raw mut msghdr,
                libc::MSG_CMSG_CLOEXEC
            ))?;
            log!(TRACE, "{:?}", msghdr);

            if msghdr.msg_controllen > 0 {
                // lol this is probably not correct, works tho
                // TODO: maybe null check this
                let cmsghdr = *libc::CMSG_FIRSTHDR(&raw const msghdr);

                if cmsghdr.cmsg_type == libc::SCM_RIGHTS {
                    let data = &ancillary_buf[size_of::<libc::cmsghdr>()..cmsghdr.cmsg_len];

                    let raw_fds =
                        core::slice::from_raw_parts(data.as_ptr().cast(), data.len() / size_of::<RawFd>());

                    for fd in raw_fds {
                        self.fds.push_back(OwnedFd::from_raw_fd(*fd));
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
            data: Bucket::new(),
            fds: VecDeque::with_capacity(MAX_FD_LEN),
            display_fd,
            _ghost: PhantomData::<Writer>,
        }
    }

    pub fn write_request(&mut self, msg: &[u8]) {
        if !self.data.can_fit(msg.len()) {
            log!(TRACE, "Buffer can't fit additional {} bytes", msg.len());
            self.send().unwrap();
        }
        self.data.extend_from_slice(msg);
    }

    pub fn add_fd(&mut self, fd: RawFd) {
        self.fds.push_back(unsafe { OwnedFd::from_raw_fd(fd) });
        log!(TRACE, "Added fd {} to pool", fd,);
    }

    fn send(&mut self) -> std::io::Result<()> {
        if self.data.empty() {
            return Ok(());
        }

        let flags = libc::MSG_NOSIGNAL;
        let len = if self.fds.is_empty() {
            let len = unsafe {
                syscall!(libc::send(
                    self.display_fd,
                    self.data.as_ptr().cast(),
                    self.data.len(),
                    flags,
                ))?
            };
            len as usize
        } else {
            unsafe {
                // we've only got 10 fds with 4B each it's 40B, CMSG_SPACE(40) = 56
                let mut buf = [0u8; Self::ALIGNED_LEN];
                let fds_len = self.fds.len() * size_of::<RawFd>();
                let required_len = libc::CMSG_SPACE(fds_len as u32) as usize;
                let buf = &mut buf[..required_len];

                let mut io = libc::iovec {
                    iov_base: self.data.as_mut_ptr().cast(),
                    iov_len: self.data.len(),
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

                (*cmsghdr).cmsg_len = libc::CMSG_LEN(fds_len as u32) as usize;
                (*cmsghdr).cmsg_level = libc::SOL_SOCKET;
                (*cmsghdr).cmsg_type = libc::SCM_RIGHTS;

                // This isn't guaranteed to return an aligned pointer
                let data = libc::CMSG_DATA(cmsghdr) as *mut RawFd;

                for (i, fd) in self.fds.iter().enumerate() {
                    data.add(i).write(fd.as_raw_fd());
                }

                // libc::memcpy(data.cast(), fds_bytes.as_ptr().cast(), fds_len);

                let len = syscall!(libc::sendmsg(self.display_fd, &raw const msghdr, flags))?;
                self.fds.clear();
                len as usize
            }
        };

        let data_len = self.data.len();
        log!(
            TRACE,
            "Written {} bytes to fd {} out of {}",
            len,
            self.display_fd,
            data_len,
        );
        debug_assert_eq!(data_len, len as usize); // ??
        unsafe {
            self.data.set_len(data_len - (len as usize));
        }
        Ok(())
    }
}

type CallbackFn<S> = fn(&mut S, &mut Connection<S>, WlEvent);

// TODO: make it actually track ids
#[allow(unused)]
#[derive(Debug)]
pub(crate) struct ObjectManager<S> {
    objects: HashMap<u32, Option<CallbackFn<S>>>,
    dead_ids: Vec<u32>,
    id_counter: IdCounter,
}

impl<S> ObjectManager<S> {
    fn new_id(&mut self) -> u32 {
        let id = if let Some(id) = self.dead_ids.pop() {
            id
        } else {
            self.id_counter.get_new()
        };
        self.objects.insert(id, None);
        id
    }

    fn set_callback(&mut self, id: u32, cb: CallbackFn<S>) {
        if let Some(old) = self.objects.get_mut(&id) {
            *old = Some(cb);
        }
    }

    fn get_callback(&self, id: u32) -> Option<CallbackFn<S>> {
        self.objects.get(&id).copied().flatten()
    }

    fn remove_callback(&mut self, id: u32) {
        if let Some(cb) = self.objects.get_mut(&id) {
            *cb = None;
        }
    }

    fn remove_id(&mut self, id: u32) {
        if let Some((id, _)) = self.objects.remove_entry(&id) {
            self.dead_ids.push(id)
        }
    }
}

pub trait Object
where
    Self: Sized,
{
    type Event<'a>;
    fn from_id(id: u32) -> Self;

    fn id(&self) -> u32;

    fn interface(&self) -> &'static str;

    fn parse_event<'a, S>(
        &self, conn: &mut Connection<S>, event: &'a crate::events::WlEvent,
    ) -> Self::Event<'a>;

    fn set_callback<S>(&self, conn: &mut Connection<S>, cb: CallbackFn<S>) {
        conn.add_callback(self, cb);
    }
}
