#![allow(unused)]

use events::*;
use std::cell::Cell;
use std::io::{Read, Write};
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::{env, io};

mod events;
use libc::{MAP_SHARED, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, c_char};

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
    pub const fn new() -> Self {
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
fn roundup(value: usize, mul: usize) -> usize {
    (((value - 1) / mul) + 1) * mul
}

#[derive(Debug)]
struct Message<const S: usize> {
    buf: [u8; S],
    len: usize,
}

impl<const S: usize> Message<S> {
    pub fn new(id: u32, op: u16) -> Self {
        let mut msg = Message::empty();
        msg.write_u32(id);
        msg.write_u16(op);
        msg.write_u16(8);
        msg
    }
    fn update_len(&mut self) {
        self.buf[6..8].copy_from_slice(&(self.len as u16).to_ne_bytes());
    }
    pub fn empty() -> Self {
        Self {
            buf: [0; S],
            len: 0,
        }
    }
    pub fn write_u32(&mut self, value: u32) {
        const SIZE: usize = size_of::<u32>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self.update_len();
    }
    pub fn write_u16(&mut self, value: u16) {
        const SIZE: usize = size_of::<u16>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self.update_len();
    }
    pub fn data(&self) -> &[u8] {
        &self.buf[..self.len]
    }
    pub fn write_str(&mut self, str: &str) {
        // null included
        self.write_u32((str.len() + 1) as u32);
        self.buf[self.len..str.len() + self.len].copy_from_slice(str.as_bytes());
        self.len += roundup(str.len() + 1, 4);
        self.update_len();
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum AppState {
    #[default]
    None,
    SurfaceAckedConfigure,
    SurfaceAttached,
}

#[derive(Debug, Default)]
struct State {
    wl_registry: u32,
    wl_shm: u32,
    wl_shm_pool: u32,
    wl_buffer: u32,
    xdg_wm_base: u32,
    xdg_surface: u32,
    wl_compositor: u32,
    wl_surface: u32,
    xdg_toplevel: u32,
    stride: u32,
    width: u32,
    height: u32,
    shm_pool_size: u32,
    shm_fd: i32,
    shm_pool_data: *mut u8,
    state: AppState,
    mapped: bool,
    exit: bool,
}

impl State {
    pub fn init_shm(&mut self) {
        self.shm_pool_size = self.stride * self.height * self.stride;
        let name = format!("/SHMTEST-{}", std::process::id());
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

    fn on_registry_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let name = parser.get_u32();
                let interface = parser.get_string();
                let version = parser.get_u32();
                println!(
                    "\x1b[32m[DEBUG]\x1b[0m: => wl_registry#{} global(name: {}, interface: {}, version: {})",
                    self.wl_registry, name, interface, version
                );

                let mut bind_reg = |name: u32, interface: &str, version: u32| -> u32 {
                    let mut msg = Message::<64>::new(self.wl_registry, 0);
                    msg.write_u32(name);
                    msg.write_str(&interface);
                    msg.write_u32(version);
                    let new_id = ID_COUNTER.get_new();
                    msg.write_u32(new_id);
                    socket.write(msg.data());
                    socket.flush();
                    println!(
                        "\x1b[32m[DEBUG]\x1b[0m: wl_registry#2.bind(name: {}, interface: {}, version: {})",
                        name, interface, version
                    );
                    new_id
                };
                match interface.as_str() {
                    "wl_shm" => {
                        self.wl_shm = bind_reg(name, &interface, version);
                    }
                    "xdg_wm_base" => {
                        self.xdg_wm_base = bind_reg(name, &interface, version);
                    }
                    "wl_compositor" => {
                        self.wl_compositor = bind_reg(name, &interface, version);
                    }
                    "wp_single_pixel_buffer_manager_v1" => {
                        self.wl_shm_pool = bind_reg(name, &interface, version);
                    }
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
            msg.write_u32(self.wl_surface);
            socket.write(msg.data());
            println!(
                "\x1b[32m[DEBUG]\x1b[0m: Created wl_surface with id #{}",
                self.wl_surface
            );
        }
        {
            let mut msg = Message::<16>::new(self.xdg_wm_base, 2);
            self.xdg_surface = ID_COUNTER.get_new();
            msg.write_u32(self.xdg_surface);
            msg.write_u32(self.wl_surface);
            socket.write(msg.data());
            println!(
                "\x1b[32m[DEBUG]\x1b[0m: Created xdg_surface with id #{}",
                self.xdg_surface
            );
        }
        {
            let mut msg = Message::<12>::new(self.xdg_surface, 1);
            self.xdg_toplevel = ID_COUNTER.get_new();
            msg.write_u32(self.xdg_toplevel);
            socket.write(msg.data());
            socket.flush();
            println!(
                "\x1b[32m[DEBUG]\x1b[0m: Created xdg_toplevel with id #{}",
                self.xdg_toplevel
            );
        }
        self.wl_surface_commit(socket);
    }

    fn wl_surface_commit(&mut self, socket: &mut UnixStream) {
        {
            let mut msg = Message::<8>::new(self.wl_surface, 6);
            socket.write(msg.data());
            socket.flush();
            println!(
                "\x1b[32m[DEBUG]\x1b[0m: wl_surface#{}.commit()",
                self.wl_surface
            );
        }
    }
    fn on_xdgbase_event(&self, socket: &mut UnixStream, event: Event<'_>) {
        match event.header.opcode {
            0 => {
                let mut parser = EventDataParser::new(event.data);
                let serial = parser.get_u32();
                println!(
                    "\x1b[32m[DEBUG]\x1b[0m: => xdg_wm_base#{}.ping(serial: {})",
                    self.xdg_wm_base, serial
                );
                let mut msg = Message::<12>::new(self.xdg_wm_base, 3);
                msg.write_u32(serial);
                socket.write(msg.data());
                socket.flush();
                println!(
                    "\x1b[32m[DEBUG]\x1b[0m: xdg_wm_base#{}.pong(serial: {})",
                    self.xdg_wm_base, serial
                );
            }
            _ => {
                println!("XDG{:?}", event.header.opcode);
            }
        }
    }

    #[rustfmt::skip]
    fn on_xdgsurface_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let serial = parser.get_u32();
                println!("\x1b[32m[DEBUG]\x1b[0m: => xdg_surface#{}.configure(serial: {})", self.xdg_surface, serial);
                let mut msg = Message::<12>::new(self.xdg_surface, 4);
                msg.write_u32(serial);
                socket.write(msg.data());
                socket.flush();
                println!("\x1b[32m[DEBUG]\x1b[0m: xdg_surface#{}.ack_configure(serial: {})", self.xdg_surface, serial);
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
                println!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.configure(width: {}, height: {}, states: {:?})", self.xdg_toplevel, width, height, states);
            }
            1 => {
                println!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.close()", self.xdg_toplevel);
                self.exit = true;
            }
            2 => {
                let width = parser.get_u32();
                let height = parser.get_u32();
                println!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.configure_bounds(width: {}, height: {})", self.xdg_toplevel, width, height);
            }
            3 => {
                println!("\x1b[32m[DEBUG]\x1b[0m: => xdg_toplevel#{}.wm_capabalites(capabalites: {:?})", self.xdg_toplevel, parser.get_array_u32());
            }
            _ => {}
        }
    }

    fn create_wl_shm_pool(&mut self, socket: &mut UnixStream) {
        let mut msg = Message::<20>::new(self.wl_shm, 0);
        self.wl_shm_pool = ID_COUNTER.get_new();
        msg.write_u32(self.wl_shm_pool);
        msg.write_u32(self.shm_fd as u32);
        msg.write_u32(self.shm_pool_size);
        socket.write(msg.data());
        socket.flush();
    }

    fn create_wl_buffer(&mut self, socket: &mut UnixStream) {
        // TODO: use wl_shm_pool actually
        let mut msg = Message::<28>::new(self.wl_shm_pool, 1);
        self.wl_buffer = ID_COUNTER.get_new();
        msg.write_u32(self.wl_buffer);
        msg.write_u32((u32::MAX / 255) * 75);
        msg.write_u32((u32::MAX / 255) * 109);
        msg.write_u32((u32::MAX / 255) * 145);
        msg.write_u32(u32::MAX);
        println!(
            "\x1b[32m\x1b[32m[DEBUG]\x1b[0m\x1b[0m: wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer(wl_buffer#{}, r: {}, g: {}, b: {}, a: {})",
            self.wl_buffer, 20, 50, 150, 1,
        );
        socket.write(msg.data());
        socket.flush();
    }

    fn wl_surface_attach(&mut self, socket: &mut UnixStream) {
        let mut msg = Message::<20>::new(self.wl_surface, 1);
        msg.write_u32(self.wl_buffer);
        msg.write_u32(0);
        msg.write_u32(0);
        println!(
            "\x1b[32m[DEBUG]\x1b[0m: wl_surface#{}(wl_buffer#{}, x: {}, y: {})",
            self.wl_surface, self.wl_buffer, 0, 0
        );
        socket.write(msg.data());
        socket.flush();
    }

    fn set_toplevel_info(&self, socket: &mut UnixStream) {
        {
            let mut msg = Message::<64>::new(self.xdg_toplevel, 2);
            let title = "YAY first wayland app";
            msg.write_str(title);
            socket.write(msg.data());
            println!(
                "\x1b[32m[DEBUG]\x1b[0m: Created xdg_toplevel#{}.set_title(title: {})",
                self.xdg_toplevel, title
            );
        }
        {
            let mut msg = Message::<64>::new(self.xdg_toplevel, 3);
            let class = "com.github.evillary";
            msg.write_str(class);
            socket.write(msg.data());
            println!(
                "\x1b[32m[DEBUG]\x1b[0m: Created xdg_toplevel#{}.set_title(title: {})",
                self.xdg_toplevel, class
            );
        }
        socket.flush();
    }

    fn on_display_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let obj_id = parser.get_u32();
                let code = parser.get_u32();
                let msg = parser.get_string();
                println!(
                    "\x1b[30m[ERROR]\x1b[0m: Fatel error from object#{}, code: {}, message: {}",
                    obj_id, code, msg
                );
                self.exit = true
            }
            1 => {
                let obj_id = parser.get_u32();
                println!(
                    "\x1b[32m[DEBUG]\x1b[0m: wl_display#1.delete_id(id: {})",
                    obj_id
                );
            }
            _ => {}
        }
    }

    fn on_wlshm_event(&mut self, socket: &mut UnixStream, event: Event<'_>) {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            0 => {
                let format = parser.get_u32();
                println!(
                    "\x1b[32m[DEBUG]\x1b[0m: => wl_shm#{}.format(format: 0x{:08x})",
                    self.wl_shm, format
                );
            }
            _ => {}
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            if self.shm_fd != 0 {
                libc::close(self.shm_fd);
            }
        }
    }
}

fn wl_get_registry(sock: &mut UnixStream) -> u32 {
    let mut msg = Message::<128>::new(1, 1);
    let reg_id = ID_COUNTER.get_new();
    msg.write_u32(reg_id);
    let _ = sock.write(msg.data());
    let _ = sock.flush();

    reg_id
}

fn main() -> io::Result<()> {
    let mut current_id = 1;
    let mut socket = get_wayland_socket()?;

    let mut state = State {
        wl_registry: wl_get_registry(&mut socket),
        height: 150,
        width: 117,
        stride: 4,
        ..Default::default()
    };
    // state.init_shm();

    // println!("{:?}", state);
    let mut buf = [0u8; 4096];
    while let Ok(read) = socket.read(&mut buf) {
        if (read == 0) {
            break;
        }

        // println!("{}", &buf[..read].escape_ascii());
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
            }
        }

        if state.wl_compositor != 0
            && state.wl_shm != 0
            && state.xdg_wm_base != 0
            && state.wl_surface == 0
        {
            state.init(&mut socket);
        }

        if state.state == AppState::SurfaceAckedConfigure && !state.mapped {
            // if state.wl_shm_pool == 0 {
            //     state.create_wl_shm_pool(&mut socket);
            // }
            if state.wl_buffer == 0 {
                state.create_wl_buffer(&mut socket);
            }
            state.wl_surface_attach(&mut socket);
            state.set_toplevel_info(&mut socket);
            state.wl_surface_commit(&mut socket);
            state.mapped = true
        }

        if (state.exit) {
            break;
        }
    }
    Ok(())
}
