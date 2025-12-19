#![allow(unused)]

use events::*;
use std::cell::Cell;
use std::ffi::{c_void, CString};
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::{env, io};

use crate::events::*;

mod events;
use libc::{
    CMSG_DATA, CMSG_FIRSTHDR, CMSG_LEN, CMSG_SPACE, MAP_SHARED, O_CREAT, O_EXCL, O_RDWR, PROT_READ,
    PROT_WRITE, SCM_RIGHTS, SOL_SOCKET, c_char, c_int, cmsghdr, iovec, msghdr, sendmsg,
};

fn get_wayland_socket() -> io::Result<UnixStream> {
    let wayland_disp = env::var_os("WAYLAND_DISPLAY").unwrap_or("wayland-0".into());
    let runtime_dir = env::var_os("XDG_RUNTIME_DIR").unwrap_or("/tmp/".into());
    UnixStream::connect(PathBuf::from(runtime_dir).join(wayland_disp))
}

static ID_COUNTER: IdCounter = IdCounter::new();

struct IdCounter {
    current: Cell<u32>,
}

unsafe impl Sync for IdCounter {}
unsafe impl Send for IdCounter {}

impl IdCounter {
    const fn new() -> Self {
        Self {
            current: Cell::new(1),
        }
    }
    pub const fn get_new(&self) -> u32 {
        let new = self.current.get() + 1;
        self.current.replace(new);
        new
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AppState {
    #[default]
    None,
    SurfaceAckedConfigure,
    SurfaceAttached,
}

#[derive(Debug, Default)]
pub struct State {
    // Globals
    pub wl_registry: u32,
    pub wl_shm: u32,
    pub wl_seat: u32,
    pub wl_shm_pool: u32,
    pub wl_cursor_mgr: u32,
    pub wl_compositor: u32,
    pub xdg_wm_base: u32,

    // shm stuff
    pub stride: u32,
    pub width: u32,
    pub height: u32,
    pub shm_pool_size: u32,
    pub shm_fd: i32,
    pub shm_pool_data: *mut u8,

    // Objects
    pub wl_buffer: u32,
    pub xdg_surface: u32,
    pub wl_surface: u32,
    pub xdg_toplevel: u32,

    pub wl_keyboard: u32,
    pub wl_pointer: u32,
    pub wl_cursorshape_dev: u32,

    pub state: AppState,
    pub mapped: bool,
    pub exit: bool,
    pub debug: bool,
}

impl State {
    pub fn init_shm(&mut self) {
        self.shm_pool_size = self.stride * self.height;
        let name = c"/shm_com_github_evillary";
        // let name: CString = {
        //     let s = format!("/SHMTEST-{}", std::process::id());
        //     CString::new(s).unwrap()
        // };
        let name = name.as_ptr() as *const c_char;
        let shm_fd = unsafe { libc::shm_open(name, O_RDWR | O_EXCL | O_CREAT, 0600) };

        if shm_fd == -1 {
            panic!("Couldn't create shm {}", io::Error::last_os_error());
        }

        unsafe {
            if libc::shm_unlink(name) == -1 {
                panic!("Couldn't unlink shm {}", io::Error::last_os_error());
            }
            if libc::ftruncate(shm_fd, self.shm_pool_size as i64) == -1 {
                panic!("Couldn't truncate shm {}", io::Error::last_os_error());
            }
        }
        let shm_pool = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                self.shm_pool_size as usize,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                shm_fd,
                0,
            )
        };

        if shm_pool == std::ptr::null_mut() {
            panic!("Couldn't mmap shm {}", io::Error::last_os_error());
        }

        self.shm_fd = shm_fd;
        self.shm_pool_data = shm_pool as *mut u8;
    }

    fn on_wlseat_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let capabalites = parser.get_u32();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_seat#{}.capabalites(cap: {})",
                        self.wl_seat, capabalites,
                    );
                }

                // wl_seat got a keyboard
                if (capabalites & 2) > 0 {
                    let mut msg = Message::<12>::new(self.wl_seat, 1);
                    self.wl_keyboard = ID_COUNTER.get_new();
                    msg.write_u32(self.wl_keyboard).build();
                    unsafe {
                        libc::send(
                            socket.as_raw_fd(),
                            msg.data() as *const [u8] as *const _,
                            msg.data().len(),
                            0,
                        )
                    };
                    if self.debug {
                        eprintln!(
                            "\x1b[32m[DEBUG]\x1b[0m: wl_seat#{}.get_keybboard(id: {})",
                            self.wl_seat, self.wl_keyboard
                        );
                    }
                }

                // wl_seat got a pointer
                if (capabalites & 1) > 0 {
                    let mut msg = Message::<12>::new(self.wl_seat, 0);
                    self.wl_pointer = ID_COUNTER.get_new();
                    msg.write_u32(self.wl_pointer).build();
                    unsafe {
                        libc::send(
                            socket.as_raw_fd(),
                            msg.data() as *const [u8] as *const _,
                            msg.data().len(),
                            0,
                        )
                    };
                    if self.debug {
                        eprintln!(
                            "\x1b[32m[DEBUG]\x1b[0m: wl_seat#{}.get_pointer(id: {})",
                            self.wl_seat, self.wl_pointer
                        );
                    }
                    if self.wl_cursor_mgr != 0 {
                        self.get_cursorshape_device(socket);
                    }
                }
            }
            1 => {}
            _ => {}
        }
    }

    fn get_cursorshape_device(&mut self, socket: &mut UnixStream) {
        let mut msg = Message::<16>::new(self.wl_cursor_mgr, 1);
        self.wl_cursorshape_dev = ID_COUNTER.get_new();
        msg.write_u32(self.wl_cursorshape_dev)
            .write_u32(self.wl_pointer)
            .build();
        unsafe {
            libc::send(
                socket.as_raw_fd(),
                msg.data() as *const [u8] as *const _,
                msg.data().len(),
                0,
            )
        };
    }

    fn on_registry_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let name = parser.get_u32();
                let interface = parser.get_string();
                let version = parser.get_u32();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_registry#{}.global(name: {}, interface: {}, version: {})",
                        self.wl_registry, name, interface, version
                    );
                }

                let mut bind_reg = |name: u32, interface: &str, version: u32| -> u32 {
                    let mut msg = Message::<64>::new(self.wl_registry, 0);
                    let new_id = ID_COUNTER.get_new();
                    msg.write_u32(name)
                        .write_str(&interface)
                        .write_u32(version)
                        .write_u32(new_id)
                        .build();
                    unsafe {
                        libc::send(
                            socket.as_raw_fd(),
                            msg.data() as *const [u8] as *const _,
                            msg.data().len(),
                            0,
                        )
                    };
                    if self.debug {
                        eprintln!(
                            "\x1b[32m[DEBUG]\x1b[0m: wl_registry#2.bind(name: {}, interface: {}, version: {})",
                            name, interface, version
                        );
                    }
                    new_id
                };
                match interface {
                    "wl_shm" => {
                        self.wl_shm = bind_reg(name, &interface, version);
                    }
                    "wl_seat" => {
                        self.wl_seat = bind_reg(name, &interface, version);
                    }
                    "xdg_wm_base" => {
                        self.xdg_wm_base = bind_reg(name, &interface, version);
                    }
                    "wl_compositor" => {
                        self.wl_compositor = bind_reg(name, &interface, version);
                    }
                    "wp_cursor_shape_manager_v1" => {
                        self.wl_cursor_mgr = bind_reg(name, &interface, version);
                    }
                    // "wp_single_pixel_buffer_manager_v1" => {
                    //     self.wl_shm_pool = bind_reg(name, &interface, version);
                    // }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn init(&mut self, socket: &mut UnixStream) {
        {
            let mut msg = Message::<12>::new(self.wl_compositor, 0);
            self.wl_surface = ID_COUNTER.get_new();
            msg.write_u32(self.wl_surface).build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };

            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: Created wl_surface with id #{}",
                    self.wl_surface
                );
            }
        }
        {
            let mut msg = Message::<16>::new(self.xdg_wm_base, 2);
            self.xdg_surface = ID_COUNTER.get_new();
            msg.write_u32(self.xdg_surface)
                .write_u32(self.wl_surface)
                .build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };

            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: Created xdg_surface with id #{}",
                    self.xdg_surface
                );
            }
        }
        {
            let mut msg = Message::<12>::new(self.xdg_surface, 1);
            self.xdg_toplevel = ID_COUNTER.get_new();
            msg.write_u32(self.xdg_toplevel).build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };
            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: Created xdg_toplevel with id #{}",
                    self.xdg_toplevel
                );
            }
        }
    }

    fn wl_surface_commit(&mut self, socket: &mut UnixStream) {
        {
            let mut msg = Message::<8>::new(self.wl_surface, 6);
            msg.build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };
            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: wl_surface#{}.commit()",
                    self.wl_surface
                );
            }
        }
    }
    fn on_xdgbase_event(&self, socket: &mut UnixStream, event: Event<'_>) {
        match event.header.opcode {
            0 => {
                let mut parser = EventDataParser::new(event.data);
                let serial = parser.get_u32();

                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => xdg_wm_base#{}.ping(serial: {})",
                        self.xdg_wm_base, serial
                    );
                }
                let mut msg = Message::<12>::new(self.xdg_wm_base, 3);
                msg.write_u32(serial).build();
                unsafe {
                    libc::send(
                        socket.as_raw_fd(),
                        msg.data() as *const [u8] as *const _,
                        msg.data().len(),
                        0,
                    )
                };

                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: xdg_wm_base#{}.pong(serial: {})",
                        self.xdg_wm_base, serial
                    );
                }
            }
            _ => {
                eprintln!("XDG{:?}", event.header.opcode);
            }
        }
    }

    #[rustfmt::skip]
    fn on_xdgsurface_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let serial = parser.get_u32();
                let mut msg = Message::<12>::new(self.xdg_surface, 4);
                msg.write_u32(serial).build();
                unsafe {
                    libc::send(
                        socket.as_raw_fd(),
                        msg.data() as *const [u8] as *const _,
                        msg.data().len(),
                        0,
                    )
                };
                if self.debug {
                    eprintln!("\x1b[32m[DEBUG]\x1b[0m: => xdg_surface#{}.configure(serial: {})", self.xdg_surface, serial);
                    eprintln!("\x1b[32m[DEBUG]\x1b[0m: xdg_surface#{}.ack_configure(serial: {})", self.xdg_surface, serial);
                }
                self.state = AppState::SurfaceAckedConfigure;
            }
            _ => {}
        }
    }

    #[rustfmt::skip]
    fn on_toplevel_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let width = parser.get_u32();
                let height = parser.get_u32();
                let states = parser.get_array_u32();
                if self.debug {
                    eprintln!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.configure(width: {}, height: {}, states: {:?})", self.xdg_toplevel, width, height, states);
                }
            }
            1 => {
                if self.debug {
                    eprintln!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.close()", self.xdg_toplevel);
                }
                self.exit = true;
            }
            2 => {
                let width = parser.get_u32();
                let height = parser.get_u32();

                if self.debug {
                    eprintln!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.configure_bounds(width: {}, height: {})", self.xdg_toplevel, width, height);
                }
            }
            3 => {
                if self.debug {
                    eprintln!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.wm_capabalites(capabalites: {:?})", self.xdg_toplevel, parser.get_array_u32());
                }
            }
            _ => unreachable!()
        }
    }

    fn create_wl_shm_pool(&mut self, socket: &mut UnixStream) {
        let mut msg = Message::<20>::new(self.wl_shm, 0);
        self.wl_shm_pool = ID_COUNTER.get_new();
        msg.write_u32(self.wl_shm_pool)
            .write_u32(self.shm_pool_size)
            .build();

        unsafe {
            // Thanks rust
            let mut buf = [0i8; unsafe { CMSG_SPACE(size_of::<i32>() as u32) as usize }];
            let mut io = iovec {
                iov_base: msg.data_mut().as_mut_ptr() as *mut _,
                iov_len: msg.data().len(),
            };
            let mut msghdr = msghdr {
                msg_iov: &mut io as *mut _,
                msg_iovlen: 1,
                msg_control: &mut buf[..] as *mut [i8] as *mut _,
                msg_controllen: buf.len(),
                msg_name: std::ptr::null_mut(),
                msg_namelen: 0,
                msg_flags: 0,
            };

            let mut cmsghdr = CMSG_FIRSTHDR(&msghdr as *const _);

            (*cmsghdr).cmsg_len = CMSG_LEN(4) as usize;
            (*cmsghdr).cmsg_level = SOL_SOCKET;
            (*cmsghdr).cmsg_type = SCM_RIGHTS;

            *(CMSG_DATA(cmsghdr) as *mut c_int) = self.shm_fd;
            msghdr.msg_controllen = CMSG_SPACE(size_of::<i32>() as u32) as usize;

            if sendmsg(socket.as_raw_fd(), &msghdr as *const msghdr, 0) == -1 {
                panic!("Failed to sendmsg, {}", std::io::Error::last_os_error());
            }
        }

        if self.debug {
            eprintln!(
                "\x1b[32m\x1b[32m[DEBUG]\x1b[0m\x1b[0m: wl_shm#{}.create_pool(wl_shm_pool#{}, fd: {}, size: {})",
                self.wl_shm, self.wl_shm_pool, self.shm_fd, self.shm_pool_size
            );
        }
        socket.flush();
        // socket.write(msg.data());
    }

    fn create_wl_buffer(&mut self, socket: &mut UnixStream) {
        // TODO: use wl_shm_pool actually
        let mut msg = Message::<32>::new(self.wl_shm_pool, 0);
        self.wl_buffer = ID_COUNTER.get_new();
        msg.write_u32(self.wl_buffer)
            .write_u32(0)
            .write_u32(self.width)
            .write_u32(self.height)
            .write_u32(self.stride)
            .write_u32(1)
            .build();
        if self.debug {
            eprintln!(
                "\x1b[32m\x1b[32m[DEBUG]\x1b[0m\x1b[0m: wl_shm_pool#{}.create_buffer(wl_buffer#{}, offset: {}, width: {}, height: {}, stride: {}, format: {})",
                self.wl_shm_pool, self.wl_buffer, 0, self.width, self.height, self.stride, 1
            );
        }
        // msg.write_u32(self.wl_buffer);
        // msg.write_u32((u32::MAX / 255) * 75);
        // msg.write_u32((u32::MAX / 255) * 109);
        // msg.write_u32((u32::MAX / 255) * 145);
        // msg.write_u32(u32::MAX);
        //
        // if self.debug {
        //     eprintln!(
        //         "\x1b[32m\x1b[32m[DEBUG]\x1b[0m\x1b[0m: wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer(wl_buffer#{}, r: {}, g: {}, b: {}, a: {})",
        //         self.wl_buffer, 20, 50, 150, 1,
        //     );
        // }
        unsafe {
            libc::send(
                socket.as_raw_fd(),
                msg.data() as *const [u8] as *const _,
                msg.data().len(),
                0,
            )
        };
        // socket.write(msg.data());
        // socket.flush();
    }

    fn wl_surface_attach(&mut self, socket: &mut UnixStream) {
        let mut msg = Message::<20>::new(self.wl_surface, 1);
        msg.write_u32(self.wl_buffer)
            .write_u32(0)
            .write_u32(0)
            .build();
        if self.debug {
            eprintln!(
                "\x1b[32m[DEBUG]\x1b[0m: wl_surface#{}(wl_buffer#{}, x: {}, y: {})",
                self.wl_surface, self.wl_buffer, 0, 0
            );
        }
        unsafe {
            libc::send(
                socket.as_raw_fd(),
                msg.data() as *const [u8] as *const _,
                msg.data().len(),
                0,
            )
        };
        // socket.flush();
    }

    fn set_toplevel_info(&self, socket: &mut UnixStream) {
        {
            let mut msg = Message::<64>::new(self.xdg_toplevel, 2);
            let title = "YAY first wayland app";
            msg.write_str(title).build();
            socket.write(msg.data());
            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: Created xdg_toplevel#{}.set_title(title: {})",
                    self.xdg_toplevel, title
                );
            }
        }
        {
            let mut msg = Message::<64>::new(self.xdg_toplevel, 3);
            let class = "com.github.evillary";
            msg.write_str(class).build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };

            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: Created xdg_toplevel#{}.set_title(title: {})",
                    self.xdg_toplevel, class
                );
            }
        }
        {
            let mut msg = Message::<64>::new(self.xdg_toplevel, 8);
            msg.write_u32(self.width).write_u32(self.height).build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };

            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: xdg_toplevel#{}.set_minsize(width: {}, height: {})",
                    self.xdg_toplevel, self.width, self.height
                );
            }
        }
        {
            let mut msg = Message::<64>::new(self.xdg_toplevel, 8);
            msg.write_u32(self.width).write_u32(self.height).build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };

            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: xdg_toplevel#{}.set_maxsize(width: {}, height: {})",
                    self.xdg_toplevel, self.width, self.height
                );
            }
        }
    }

    fn on_display_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let obj_id = parser.get_u32();
                let code = parser.get_u32();
                let msg = parser.get_string();
                if self.debug {
                    eprintln!(
                        "\x1b[31m[ERROR]\x1b[0m: Fatel error from object#{}, code: {}, message: {}",
                        obj_id, code, msg
                    );
                }
                self.exit = true
            }
            1 => {
                let obj_id = parser.get_u32();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: wl_display#1.delete_id(id: {})",
                        obj_id
                    );
                }
            }
            _ => unreachable!(),
        }
    }

    // fn cleanup(&self, socket: &mut UnixStream) {
    //     if
    // }

    fn on_wlshm_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let format = parser.get_u32();

                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_shm#{}.format(format: 0x{:08x})",
                        self.wl_shm, format
                    );
                }
            }
            _ => unreachable!(),
        }
    }

    fn set_cursor_shape(&self, socket: &mut UnixStream, shape: u32, serial: u32) {
        let mut msg = Message::<16>::new(self.wl_cursorshape_dev, 1);
        msg.write_u32(serial).write_u32(shape).build();
        unsafe {
            libc::send(
                socket.as_raw_fd(),
                msg.data() as *const [u8] as *const _,
                msg.data().len(),
                0,
            )
        };
    }
    fn on_wlpointer_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        // eprintln!("{:?}", event.header);
        match event.header.opcode {
            0 => {
                let serial = parser.get_u32();
                let surface = parser.get_u32();
                let surface_x = parser.get_fixed();
                let surface_y = parser.get_fixed();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_pointer#{}.enter(serial: {serial}, surface: {surface}, x: {:.2}, y: {:.2})",
                        self.wl_pointer, surface_x, surface_y
                    );
                }

                if self.wl_cursorshape_dev != 0 {
                    // 8 - crosshair
                    self.set_cursor_shape(socket, 8, serial);
                }
            }
            1 => {
                let serial = parser.get_u32();
                let surface = parser.get_u32();

                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_pointer#{}.leave(serial: {serial}, surface: {surface})",
                        self.wl_pointer
                    );
                }
            }
            2 => {
                let time = parser.get_u32();
                let surface_x = parser.get_fixed();
                let surface_y = parser.get_fixed();
                // Spam

                // if self.debug {
                //     eprintln!(
                //         "\x1b[32m[DEBUG]\x1b[0m: => wl_pointer#{}.motion(time: {time}, x: {:.2}, y: {:.2})",
                //         self.wl_pointer, surface_x, surface_y
                //     );
                // }
            }
            _ => {}
        }
    }
    fn on_wlkeyboard_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            // keymap
            0 => {
                // TODO: actually recieve file descriptors
                // eprintln!(
                //     "\x1b[32m[DEBUG]\x1b[0m: => wl_shm#{}.format(format: 0x{:08x})",
                //     self.wl_shm, format
                // );
            }
            // enter,
            1 => {
                let serial = parser.get_u32();
                let surface = parser.get_u32();
                let keys = parser.get_array_u32();

                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_keyboard#{}.enter(serial: {}, surface: {}, keys: {:?})",
                        self.wl_keyboard, serial, surface, keys
                    );
                }
            }
            // leave
            2 => {
                let serial = parser.get_u32();
                let surface = parser.get_u32();

                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_keyboard#{}.leave(serial: {}, surface: {})",
                        self.wl_keyboard, serial, surface
                    );
                }
            }
            // key
            3 => {
                let serial = parser.get_u32();
                let time = parser.get_u32();
                let key = parser.get_u32();
                let state = parser.get_u32();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_keyboard#{}.key(serial: {}, time: {}, key: {}, state: {})",
                        self.wl_keyboard, serial, time, key, state
                    );
                }
                // q key, and escape. These are keyscan similar to the kernel's
                if key == 16 || key == 1 {
                    self.exit = true
                }
            }
            // modifiers
            4 => {
                let serial = parser.get_u32();
                let mods_depressed = parser.get_u32();
                let mods_latched = parser.get_u32();
                let mods_locked = parser.get_u32();
                let group = parser.get_u32();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_keyboard#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {}, group: {})",
                        self.wl_keyboard, serial, mods_depressed, mods_latched, mods_locked, group
                    );
                }
            }
            // repeat_info
            5 => {
                let rate = parser.get_u32();
                let delay = parser.get_u32();
                if self.debug {
                    eprintln!(
                        "\x1b[32m[DEBUG]\x1b[0m: => wl_keyboard#{}.repeat_info(rate: {}, delay: {})",
                        self.wl_keyboard, rate, delay
                    );
                }
            }
            _ => unreachable!(),
        }
    }

    fn wl_surface_damage(&self, socket: &mut UnixStream) {
        {
            let mut msg = Message::<24>::new(self.wl_surface, 2);
            msg.write_u32(0)
                .write_u32(0)
                .write_u32(self.width)
                .write_u32(self.height)
                .build();
            unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    msg.data() as *const [u8] as *const _,
                    msg.data().len(),
                    0,
                )
            };
            if self.debug {
                eprintln!(
                    "\x1b[32m[DEBUG]\x1b[0m: wl_surface#{}.damage(x: {}, y: {}, w: {}, h: {})",
                    self.wl_surface, 0, 0, self.width, self.height
                );
            }
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            if self.shm_fd != 0 {
                libc::close(self.shm_fd);
            }
            if self.shm_pool_data != core::ptr::null_mut() {
                libc::munmap(
                    self.shm_pool_data as *mut c_void,
                    core::mem::size_of_val(self.shm_pool_data.as_mut().unwrap()),
                );
            }
        }
    }
}

fn wl_get_registry(sock: &mut UnixStream) -> u32 {
    let mut msg = Message::<128>::new(1, 1);
    let reg_id = ID_COUNTER.get_new();
    msg.write_u32(reg_id).build();
    let _ = sock.write(msg.data());
    let _ = sock.flush();

    reg_id
}

fn main() -> io::Result<()> {
    let mut current_id = 1;
    let mut socket = get_wayland_socket()?;

    let mut state = State {
        wl_registry: wl_get_registry(&mut socket),
        height: 500,
        width: 500,
        stride: 4 * 500, // bytes per row
        ..Default::default()
    };

    if let Some(_) = env::var_os("WAYLAND_DEBUG") {
        state.debug = true;
    }

    state.init_shm();

    // state.debug = true;

    let w = state.width as usize;
    let h = state.height as usize;
    let cx = state.width / 2;
    let cy = state.height / 2;

    let mut pixels = unsafe {
        std::slice::from_raw_parts_mut(
            state.shm_pool_data as *mut u32,
            state.shm_pool_size as usize / 4,
        )
    };

    for (y, row) in pixels.chunks_mut(w).enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            // let dx: i32 = cx as i32 - x as i32;
            // let dy: i32 = cy as i32 - y as i32;
            // let color = if dx * dx + dy * dy < 4900 { 0xFFB321 } else { 0x96ABC1 };
            // *cell = color;
            let r = ((x * 255) / w) as u32;
            let g = 0;
            // let g = (((x + w) / h) * 255) as u32;
            let b = ((y * 255) / h) as u32;
            // let g = ((y * 255) / w) as u32;
            // let b = 0;
            // let r = ((x / (w - 1)) * 255) as u32;
            // let g = ((y / (h - 1)) * 255) as u32;
            *cell = (r << 16) | (g << 8) | b;
        }
    }
    let mut buf = [0u8; 4096];
    while let Ok(read) = socket.read(&mut buf) {
        if (read == 0) {
            break;
        }

        let mut event_iter = EventIter::new(&buf[..read]);
        while let Some(event) = event_iter.next() {
            if event.header.id == state.wl_registry {
                state.on_registry_event(&mut socket, event);
            } else if event.header.id == state.xdg_wm_base {
                state.on_xdgbase_event(&mut socket, event);
            } else if event.header.id == state.wl_shm {
                state.on_wlshm_event(&mut socket, event);
            } else if event.header.id == state.xdg_toplevel {
                state.on_toplevel_event(&mut socket, event);
            } else if event.header.id == state.xdg_surface {
                state.on_xdgsurface_event(&mut socket, event);
            } else if event.header.id == 1 {
                state.on_display_event(&mut socket, event);
            } else if event.header.id == state.wl_seat {
                state.on_wlseat_event(&mut socket, event);
            } else if event.header.id == state.wl_keyboard {
                state.on_wlkeyboard_event(&mut socket, event);
            } else if event.header.id == state.wl_pointer {
                state.on_wlpointer_event(&mut socket, event);
            } else {
                eprintln!(
                    "\x1b[33m[WARNING]\x1b[0m: Unhandled event: {:?}",
                    event.header
                );
            }
        }

        if state.wl_compositor != 0
            && state.wl_shm != 0
            && state.xdg_wm_base != 0
            && state.wl_surface == 0
            && state.state == AppState::None
        {
            state.init(&mut socket);
            state.set_toplevel_info(&mut socket);
            state.wl_surface_commit(&mut socket);
        }

        if state.state == AppState::SurfaceAckedConfigure && !state.mapped {
            if state.wl_shm_pool == 0 {
                state.create_wl_shm_pool(&mut socket);
            }
            if state.wl_buffer == 0 {
                state.create_wl_buffer(&mut socket);
            }

            state.wl_surface_attach(&mut socket);
            state.wl_surface_commit(&mut socket);
            // state.wl_surface_damage(&mut socket);
            state.mapped = true;
            state.state = AppState::SurfaceAttached;
        }

        socket.flush();

        if (state.exit) {
            break;
        }
    }
    Ok(())
}
