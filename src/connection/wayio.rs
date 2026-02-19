const MAX_BUFFER_SIZE: usize = 4096;
const MAX_FD_LEN: usize = 12;
const ALIGNED_LEN: usize = unsafe { libc::CMSG_SPACE((MAX_FD_LEN * size_of::<RawFd>()) as u32) as usize };

pub(super) struct WaylandBuffer<T> {
    pub(super) data: Bucket<u8, MAX_BUFFER_SIZE>,
    pub(super) fds: VecDeque<OwnedFd>,
    _ghost: PhantomData<T>,
}

impl<T> core::fmt::Debug for WaylandBuffer<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WaylandBuffer")
            .field("data", &self.data)
            .field("fds", &self.fds)
            .finish()
    }
}

pub(super) struct Reader;
impl WaylandBuffer<Reader> {
    pub(super) fn new() -> WaylandBuffer<Reader> {
        let mut data = Bucket::new();
        data.fill(0);
        Self {
            data,
            fds: VecDeque::with_capacity(MAX_FD_LEN),
            _ghost: PhantomData,
        }
    }

    pub(super) fn get_fd(&mut self) -> Option<OwnedFd> {
        self.fds.pop_front()
    }

    pub(super) fn recv(&mut self, display_fd: RawFd) -> io::Result<usize> {
        let mut ancillary_buf = [0u8; ALIGNED_LEN];

        self.fds.clear();
        unsafe {
            let mut msghdr: libc::msghdr = core::mem::zeroed();

            msghdr.msg_name = std::ptr::null_mut();
            msghdr.msg_namelen = 0;

            let mut iov = libc::iovec {
                iov_base: self.data.as_mut_ptr().cast(),
                iov_len: self.data.len(),
            };
            msghdr.msg_iov = &raw mut iov;
            msghdr.msg_iovlen = 1;

            msghdr.msg_controllen = ancillary_buf.len();
            msghdr.msg_control = ancillary_buf.as_mut_ptr().cast();

            let len = syscall!(libc::recvmsg(display_fd, &raw mut msghdr, libc::MSG_CMSG_CLOEXEC))?;

            // TODO: handle this probably
            log!(TRACE, "Read {} bytes from server", len);
            if len == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "Connection closed by peer",
                ));
            }

            if msghdr.msg_controllen > 0 {
                // lol this is probably not correct, works tho
                // TODO: maybe check this pointer for alignment and such
                let cmsghdr = libc::CMSG_FIRSTHDR(&raw const msghdr);
                debug_assert!(!cmsghdr.is_null());

                if (*cmsghdr).cmsg_type == libc::SCM_RIGHTS {
                    let data = &ancillary_buf[size_of::<libc::cmsghdr>()..(*cmsghdr).cmsg_len];

                    let raw_fds =
                        core::slice::from_raw_parts(data.as_ptr().cast(), data.len() / size_of::<RawFd>());

                    for fd in raw_fds {
                        self.fds.push_back(OwnedFd::from_raw_fd(*fd));
                    }

                    log!(TRACE, "Recived ancillay data: {:?}", cmsghdr);
                }
            }
            log!(TRACE, "Recieved {} bytes from Wayland Server", len);

            Ok(len as usize)
        }
    }
}

pub(super) struct Writer;
impl WaylandBuffer<Writer> {
    pub(super) fn new() -> WaylandBuffer<Writer> {
        Self {
            data: Bucket::new(),
            fds: VecDeque::with_capacity(MAX_FD_LEN),
            _ghost: PhantomData::<Writer>,
        }
    }

    pub(super) fn send(&mut self, display_fd: RawFd) -> io::Result<()> {
        if self.data.is_empty() {
            return Ok(());
        }

        let flags = libc::MSG_NOSIGNAL;
        let len = if self.fds.is_empty() {
            let len = unsafe {
                syscall!(libc::send(
                    display_fd,
                    self.data.as_ptr().cast(),
                    self.data.len(),
                    flags,
                ))?
            };
            len as usize
        } else {
            unsafe {
                // we've only got 10 fds with 4B each it's 40B, CMSG_SPACE(40) = 56
                let mut buf = [0u8; ALIGNED_LEN];
                let fds_len = self.fds.len() * size_of::<RawFd>();
                let required_len = libc::CMSG_SPACE(fds_len as u32) as usize;
                let buf = &mut buf[..required_len];

                let mut iov = libc::iovec {
                    iov_base: self.data.as_mut_ptr().cast(),
                    iov_len: self.data.len(),
                };

                let msghdr = libc::msghdr {
                    msg_iov: &raw mut iov,
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

                let len = syscall!(libc::sendmsg(display_fd, &raw const msghdr, flags))?;
                self.fds.clear();
                len as usize
            }
        };

        let data_len = self.data.len();
        log!(
            TRACE,
            "Written {} bytes to Wayland Server out of {}",
            len,
            data_len,
        );
        debug_assert_eq!(data_len, len as usize); // ??
        unsafe {
            self.data.set_len(data_len - (len as usize));
        }
        Ok(())
    }
}

use crate::{
    log,
    utils::{Bucket, syscall},
};
use std::{
    collections::VecDeque,
    io,
    marker::PhantomData,
    os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
};
