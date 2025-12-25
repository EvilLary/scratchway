#![allow(unused)]
use std::path::PathBuf;
use std::{
    cell::Cell,
    env,
    ffi::c_void,
    io::{self, Read, Write},
    os::{
        fd::AsRawFd,
        unix::net::UnixStream,
    },
};

use libc::{
    CMSG_DATA, CMSG_FIRSTHDR, CMSG_LEN, CMSG_SPACE, MAP_SHARED, O_CREAT, O_EXCL, O_RDWR, PROT_READ,
    PROT_WRITE, SCM_RIGHTS, SOL_SOCKET, c_char, c_int, iovec, msghdr, sendmsg,
};

// struct Connection {
//     socket: UnixStream,
// }

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
        let shm_fd = unsafe { libc::shm_open(name, O_RDWR | O_EXCL | O_CREAT, 0o600) };

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

        if shm_pool.is_null() {
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
            1 => {},
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
                msg.data().as_ptr().cast(),
                msg.data().len(),
                0,
            )
        };
        // socket.write(msg.data());
        // socket.flush();
    }

    fn wl_surface_attach(&mut self, socket: &mut UnixStream) {
        let mut msg = Message::<12>::new(self.wl_surface, 7);
        msg.write_u32(6)
            .build();
        if self.debug {
            eprintln!(
                "\x1b[32m[DEBUG]\x1b[0m: wl_surface#{}.set_buffer_transform(transform: {})",
                self.wl_surface, 6
            );
        }
        unsafe {
            libc::send(
                socket.as_raw_fd(),
                msg.data().as_ptr().cast(),
                msg.data().len(),
                0,
            )
        };

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
                msg.data().as_ptr().cast(),
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
                eprintln!(
                    "\x1b[31m[ERROR]\x1b[0m: Fatel error from object#{}, code: {}, message: {}",
                    obj_id, code, msg
                );
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
        let parser = EventDataParser::new(event.data);
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
        let parser = EventDataParser::new(event.data);
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
            if !self.shm_pool_data.is_null() {
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
    let mut socket = get_wayland_socket()?;

    let mut state = State {
        wl_registry: wl_get_registry(&mut socket),
        height: 800,
        width: 800,
        stride: 4 * 800, // bytes per row
        ..Default::default()
    };

    unsafe {
        let arg = libc::getenv(c"WAYLAND_DEBUG".as_ptr());
        if !arg.is_null() {
            state.debug = true
        }
    }
    state.init_shm();

    // state.debug = true;

    let w = state.width as usize;
    let h = state.height as usize;
    // let cx = state.width / 2;
    // let cy = state.height / 2;

    let pixels = unsafe {
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
            // let r = ((x * 255) / w) as u32;
            // let r = (((w-x) * 0xFF) / w).min( ((h-y) * 0xFF) / h) as u32;
            // let g = 0;
            // let g = (((x + w) / h) * 255) as u32;
            let b = ((y * 255) / h) as u32;
            let r = (((w - x) * 0xFF) / w).min(((h - y) * 0xFF) / h) as u32;
            let g = ((x * 0xFF) / w).min(((h-y)*0xFF)/ h) as u32;
            // let b = min(((w - x) * 0xFF) / w, (y * 0xFF) / h);
            // let g = ((y * 255) / w) as u32;
            // let b = 0;
            // let r = ((x / (w - 1)) * 255) as u32;
            // let g = ((y / (h - 1)) * 255) as u32;
            *cell = (r << 16) | (g << 8) | b;
        }
    }
    //
    // struct Rect {
    //     x: usize,
    //     y: usize,
    //     w: usize,
    //     h: usize,
    // };
    //
    // let rect = Rect {
    //     x: 450,
    //     y: 450,
    //     w: 100,
    //     h: 200,
    // };

    // let max_x = rect.x + rect.w;
    // let max_y = rect.y + rect.h;

    // for y in (rect.y..max_y) {
    //     for x in (rect.x..max_x) {
    //         pixels[y * (state.width as usize) + x] = 0x0000FFFF;
    //     }
    // }

    struct Circle {
        x: usize,
        y: usize,
        r: usize
    }

    let circle = Circle {
        x: 100,
        y: 100,
        r: 100,
    };

    let begin_x = circle.x - circle.r;
    let begin_y = circle.y - circle.r;

    let end_x = circle.x + circle.r;
    let end_y = circle.y + circle.r;

    println!("Bx: {begin_x}, By: {begin_y}, Ex: {end_x}, Ey: {end_y}");
    let r_seq = (circle.r * circle.r) as i32;
    // let w = (circle.r * 2) as i32;
    // let h = (circle.r * 2) as i32;
    for y in begin_y..end_y {
        for x in begin_x..end_x {
            let dx = circle.x as i32 - x as i32;
            let dy = circle.y as i32 - y as i32;
            if (dx * dx) + (dy * dy) < r_seq {
                // pixels[y * (state.width as usize) + x] = (r << 16) | (g << 8) | b;
                pixels[y * (state.width as usize) + x] = 0xff00ff;
            }
        }
    }

    // for (y, row) in &mut pixels
    //     [rect.y * (state.width as usize)..(rect.y + rect.h) * (state.width as usize)]
    //     .chunks_mut(state.width as usize)
    //     .skip(rect.y).enumerate()
    // {
    //     for p in &mut row[rect.x..rect.x + rect.w] {
    //         *p = 0xFFff0F;
    //     }
    // }
    // for (y, row) in pixels
    //     .chunks_mut(state.width as usize)
    //     .skip(rect.y)
    //     .enumerate()
    // {
    //     for p in &mut row[rect.x..rect.x + rect.w] {
    //         *p = 0xFF00FF;
    //     }
    // }
    // for p in pixels
    //     .iter_mut()
    //     .skip((rect.y * (state.width as usize)) + rect.x)
    // {
    //     *p = 0xFF00FF;
    // }
    // for (y, row) in pixels.chunks_mut(w).enumerate() {
    //     for (x, cell) in row.iter_mut().enumerate() {
    //         // let dx: i32 = cx as i32 - x as i32;
    //         // let dy: i32 = cy as i32 - y as i32;
    //         // let color = if dx * dx + dy * dy < 4900 { 0xFFB321 } else { 0x96ABC1 };
    //         // *cell = color;
    //         let r = ((x * 255) / w) as u32;
    //         let g = 0;
    //         // let g = (((x + w) / h) * 255) as u32;
    //         let b = ((y * 255) / h) as u32;
    //         // let g = ((y * 255) / w) as u32;
    //         // let b = 0;
    //         // let r = ((x / (w - 1)) * 255) as u32;
    //         // let g = ((y / (h - 1)) * 255) as u32;
    //         *cell = (r << 16) | (g << 8) | b;
    //     }
    // }

    let mut buf = [0u8; 4096];
    while let Ok(read) = socket.read(&mut buf) {
        if read == 0 {
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

        socket.flush()?;

        if state.exit {
            break;
        }
    }
    Ok(())
}


#[derive(Debug)]
pub struct EventIter<'a> {
    buf: &'a [u8],
}

impl<'a> EventIter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        EventIter { buf: data }
    }
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // let header_size = Header::HEADER_SIZE;
        if self.buf.len() < Header::HEADER_SIZE {
            return None;
        }
        let header = &self.buf[0..Header::HEADER_SIZE];
        let header = Header::from_slice(header);

        if self.buf.len() < header.size as usize {
            return None;
        }

        let data = &self.buf[Header::HEADER_SIZE..header.size as usize];

        if self.buf.len() <= header.size as usize {
            self.buf = &[];
        } else {
            self.buf = &self.buf[header.size as usize..];
        }
        Some(Event { header, data })
    }
}

#[derive(Debug)]
pub struct Event<'a> {
    pub header: Header,
    pub data: &'a [u8],
}

impl<'a> Event<'a> {
    pub fn parser(&self) -> EventDataParser<'a> {
        EventDataParser::new(self.data)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    pub id: u32,
    pub opcode: u16,
    pub size: u16,
}

impl Header {
    pub const HEADER_SIZE: usize = size_of::<Self>();
    pub fn new(id: u32, opcode: u16, size: u16) -> Self {
        Self { id, opcode, size }
    }
    pub fn from_slice(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), std::mem::size_of::<Self>());
        let id = u32::from_ne_bytes([slice[0], slice[1], slice[2], slice[3]]);
        let opcode = u16::from_ne_bytes([slice[4], slice[5]]);
        let size = u16::from_ne_bytes([slice[6], slice[7]]);
        Self { id, opcode, size }
    }
}

pub struct EventDataParser<'a> {
    pub data: &'a [u8],
    idx: Cell<usize>,
}

impl<'a> EventDataParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            idx: Cell::new(0),
        }
    }

    pub fn get_u16(&mut self) -> u16 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = u16::from_ne_bytes([data[0], data[1]]);
        self.idx.replace(idx + core::mem::size_of::<u16>());
        num
    }

    pub fn get_fixed(&mut self) -> f32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]) as f32;
        self.idx.replace(idx + core::mem::size_of::<i32>());
        num / 256.0
    }

    pub fn get_u32(&self) -> u32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        // let num = u32::from_ne_bytes(data[0..4]);
        let num = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        self.idx.replace(idx + core::mem::size_of::<u32>());
        num
    }

    pub fn get_string<'b>(&'a self) -> &'b str {
        let str_len = self.get_u32() as usize;
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let padded_len = roundup(str_len, 4);
        // Null terminator not included
        let str = &data[..str_len - 1];
        self.idx.replace(idx + padded_len);

        // SAFETY: the reference behind message is valid for as long
        // as the event.data is valid, Rust just can't know it
        unsafe {
            let len = str.len();
            let ptr = str.as_ptr();
            let slice = core::slice::from_raw_parts(ptr, len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    pub fn get_array_u32<'b>(&'a self) -> &'b [u32] {
        let array_len = self.get_u32() as usize;
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let array = unsafe {
            let ptr = data[..array_len].as_ptr() as *const u32;
            core::slice::from_raw_parts(ptr, array_len / size_of::<u32>())
        };
        self.idx.replace(idx + array_len);
        array
    }

    pub(crate) fn get_i32(&self) -> i32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        self.idx.replace(idx + core::mem::size_of::<i32>());
        num
    }
}

fn roundup(value: usize, mul: usize) -> usize {
    (((value - 1) / mul) + 1) * mul
}

#[derive(Debug)]
pub struct Message<const S: usize> {
    buf: [u8; S],
    len: usize,
}

impl<const S: usize> Message<S> {
    pub fn new(id: u32, op: u16) -> Self {
        let mut msg = Message::empty();
        msg.write_u32(id).write_u16(op).write_u16(8);
        msg
    }

    pub fn build(&mut self) {
        self.buf[6..8].copy_from_slice(&(self.len as u16).to_ne_bytes());
    }

    fn empty() -> Self {
        Self {
            buf: [0; S],
            len: 0,
        }
    }

    pub fn write_i32(&mut self, value: i32) -> &mut Self {
        const SIZE: usize = size_of::<i32>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self
    }

    pub fn write_u32(&mut self, value: u32) -> &mut Self {
        const SIZE: usize = size_of::<u32>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self
    }

    // TODO: Does this actually work??
    pub fn write_fixed(&mut self, value: f32) -> &mut Self {
        let wl_fixed = f32::to_bits((value * 256.0).round());
        self.write_u32(wl_fixed);
        self
    }

    pub fn write_u16(&mut self, value: u16) -> &mut Self {
        const SIZE: usize = size_of::<u16>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self
    }

    pub fn write_str(&mut self, str: &str) -> &mut Self {
        // null included
        self.write_u32((str.len() + 1) as u32);
        self.buf[self.len..str.len() + self.len].copy_from_slice(str.as_bytes());
        self.len += roundup(str.len() + 1, 4);
        self
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..self.len]
    }

    pub fn data(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
