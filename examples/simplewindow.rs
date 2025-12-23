#![allow(unused)]

use std::os::fd::RawFd;
use std::time::Instant;

use scratchway::connection::Connection;
use scratchway::events::Event;
use scratchway::protocols::core::*;
use scratchway::protocols::viewporter::*;
use scratchway::protocols::xdg_shell::*;
use scratchway::protocols::wlr_layer_shell_unstable_v1::*;

#[derive(Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

fn main() -> std::io::Result<()> {
    let conn = Connection::connect()?;

    let rect = Rect {
        x: 50,
        y: 50,
        w: 50,
        h: 50,
    };

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&conn);

    let mut callbacks: Vec<(u32, Callback)> = Vec::with_capacity(16);

    callbacks.push((wl_registry.id(), State::on_registry_event));
    callbacks.push((wl_display.id(), State::on_wldisplay_event));

    let mut state = State {
        wl_display,
        wl_registry: Some(wl_registry),
        callbacks,
        dy: 5,
        dx: 10,
        rect,
        ..Default::default()
    };

    while !state.exit {
        let events = conn.blocking_read();
        for event in events {
            state.handle_event(&conn, event);
        }
    }

    state.cleanup();

    Ok(())
}

type Callback = fn(&mut State, &Connection, Event<'_>);
#[derive(Debug, Default)]
struct State {
    callbacks: Vec<(u32, Callback)>,

    wl_display: WlDisplay,
    wl_registry: Option<WlRegistry>,
    wl_compositor: Option<WlRegistry>,
    xdg_wm_base: Option<XdgWmBase>,
    viewporter: Option<WpViewporter>,

    wl_surface: Option<WlSurface>,
    wl_buffer: Option<WlBuffer>,
    xdg_toplevel: Option<XdgToplevel>,
    xdg_surface: Option<XdgSurface>,
    viewport: Option<WpViewport>,
    configured: bool,

    wl_shm: Option<WlShm>,
    wl_shm_pool: Option<WlShmPool>,
    shm_fd: RawFd,
    shm_data: *mut u8,
    shm_pool_size: i32,
    width: i32,
    height: i32,
    stride: i32,

    window_height: i32,
    window_width: i32,
    window_size_changed: bool,

    dx: i32,
    dy: i32,
    rect: Rect,

    exit: bool,
}

impl State {
    pub fn handle_event(&mut self, conn: &Connection, event: Event<'_>) {
        if let Some((_, cb)) = self.callbacks.iter().find(|(id, _)| *id == event.header.id) {
            cb(self, conn, event)
        } else {
            eprintln!(
                "[\x1b[33mWARNING\x1b[0m]: Unhandled event for id: {}, opcode: {}",
                event.header.id, event.header.opcode
            )
        }
    }

    fn on_wldisplay_event(&mut self, _conn: &Connection, event: Event) {
        match self.wl_display.parse_event(event) {
            WlDisplayEvent::Error {
                object_id,
                code,
                message,
            } => {
                eprintln!("Protocol error: code {code} from object {object_id}, {message}");
                self.exit = true;
            }
            WlDisplayEvent::DeleteId { id } => self.callbacks.retain(|(obj_id, _)| id != *obj_id),
        }
    }

    fn on_registry_event(&mut self, conn: &Connection, event: Event) {
        let Some(wl_registry) = self.wl_registry.as_ref() else {
            return; // this should never be reached
        };
        match wl_registry.parse_event(event) {
            WlRegistryEvent::Global {
                name,
                interface,
                version,
            } => match interface {
                "wp_viewporter" => {
                    self.viewporter = Some(wl_registry.bind(&conn, name, interface, version));
                }
                "wl_shm" => {
                    let wl_shm: WlShm = wl_registry.bind(&conn, name, interface, version);
                    self.wl_shm = Some(wl_shm);
                    self.init_shm(conn);
                }
                "wl_compositor" => {
                    let wl_compositor: WlCompositor =
                        wl_registry.bind(&conn, name, interface, version);
                    let wl_surface = wl_compositor.create_surface(conn);

                    self.callbacks
                        .push((wl_surface.id, Self::on_wlsurface_event));
                    self.wl_surface = Some(wl_surface);
                }
                "xdg_wm_base" => {
                    let xdg_wm_base: XdgWmBase = wl_registry.bind(&conn, name, interface, version);
                    self.callbacks
                        .push((xdg_wm_base.id, Self::on_xdgwmbase_event));

                    self.xdg_wm_base = Some(xdg_wm_base);
                    if self.xdg_wm_base.is_some() && self.xdg_surface.is_none() {
                        self.init_toplevel(conn);
                    }
                }
                _ => {}
            },
            WlRegistryEvent::GlobalRemove { name } => {
                println!("Removed: {:?}", name);
            }
        }
    }

    fn on_xdgsurface_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(xdg_surface) = self.xdg_surface.as_ref() else {
            return;
        };
        match xdg_surface.parse_event(event) {
            XdgSurfaceEvent::Configure { serial } => {
                let wl_surface = self.wl_surface.as_ref().unwrap();
                if let Some(ref wl_buffer) = self.wl_buffer
                    && !self.configured
                {
                    // wl_surface.set_buffer_scale(conn, 2);
                    // wl_surface.set_buffer_transform(conn, WlOutputTransform::Flipped270);
                    wl_surface.attach(conn, Some(wl_buffer), 0, 0);
                    wl_surface.commit(conn);
                    self.configured = true;
                }

                if self.window_size_changed {
                    xdg_surface.set_window_geometry(
                        conn,
                        0,
                        0,
                        self.window_width,
                        self.window_height,
                    );
                    if let Some(ref viewport) = self.viewport {
                        viewport.set_destination(conn, self.window_width, self.window_height);
                    }
                    // wl_surface.damage_buffer(conn, 0, 0, self.window_height, self.window_height);
                    wl_surface.commit(conn);
                    self.window_size_changed = false;
                }

                xdg_surface.ack_configure(conn, serial);
            }
        }
    }

    fn on_wlsurface_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(wl_surface) = self.wl_surface.as_ref() else {
            return;
        };
        match wl_surface.parse_event(event) {
            WlSurfaceEvent::Enter { output } => {}
            WlSurfaceEvent::Leave { output } => {}
            WlSurfaceEvent::PrefferedBufferScale { factor } => {}
            WlSurfaceEvent::PrefferedBufferTransform { transform } => {}
        }
    }

    fn on_xdgwmbase_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(xdg_wm_base) = self.xdg_wm_base.as_ref() else {
            return;
        };
        match xdg_wm_base.parse_event(event) {
            XdgWmBaseEvent::Ping { serial } => {
                xdg_wm_base.pong(conn, serial);
                // self.draw(conn);
            }
        }
    }

    fn on_xdgtoplevel_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(xdg_toplevel) = self.xdg_toplevel.as_ref() else {
            return;
        };
        match xdg_toplevel.parse_event(event) {
            XdgToplevelEvent::Configure {
                width,
                height,
                states,
            } => {
                if width != 0 && height != 0 {
                    if self.window_width != width || self.window_height != height {
                        self.window_height = height;
                        self.window_width = width;
                        self.window_size_changed = true;
                    }
                }
            }
            XdgToplevelEvent::ConfigureBounds { width, height } => {}
            XdgToplevelEvent::Close => self.exit = true,
            XdgToplevelEvent::WmCapabilities { capabilities } => {}
        }
    }

    fn on_wlbuffer_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            return; // this should never be reached
        };
        match wl_buffer.parse_event(event) {
            WlBufferEvent::Release => {
                // self.draw(conn);
                // std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }
    }

    fn init_toplevel(&mut self, conn: &Connection) {
        let Some(wl_surface) = self.wl_surface.as_ref() else {
            return;
        };

        let Some(xdg_wm_base) = self.xdg_wm_base.as_ref() else {
            return;
        };

        let xdg_surface = xdg_wm_base.get_xdg_surface(conn, wl_surface);
        self.callbacks
            .push((xdg_surface.id, Self::on_xdgsurface_event));

        let xdg_toplevel = xdg_surface.get_toplevel(conn);
        self.callbacks
            .push((xdg_toplevel.id, Self::on_xdgtoplevel_event));

        xdg_toplevel.set_title(conn, "Hola bola");
        xdg_toplevel.set_app_id(conn, "com.github.evillary");

        if let Some(ref viewporter) = self.viewporter {
            let viewport = viewporter.get_viewport(conn, wl_surface);
            viewport.set_destination(conn, self.width, self.height);
            self.viewport = Some(viewport);
        }

        // wl_surface.set_buffer_scale(conn, 2);
        xdg_surface.set_window_geometry(conn, 0, 0, self.width, self.height);
        wl_surface.attach(conn, self.wl_buffer.as_ref(), 0, 0);
        wl_surface.commit(conn);

        self.xdg_toplevel = Some(xdg_toplevel);
        self.xdg_surface = Some(xdg_surface);
    }

    fn init_shm(&mut self, conn: &Connection) {
        self.height = 400;
        self.width = 400;
        self.stride = self.width * 4;

        self.shm_pool_size = self.stride * self.height;
        let name = c"/shm_com_github_evillary".as_ptr().cast();
        let shm_fd = unsafe {
            let flags = libc::O_RDWR | libc::O_EXCL | libc::O_CREAT;
            libc::shm_open(name, flags, 0600)
        };

        if shm_fd == -1 {
            panic!("Couldn't create shm {}", std::io::Error::last_os_error());
        }

        unsafe {
            if libc::shm_unlink(name) == -1 {
                panic!("Couldn't unlink shm {}", std::io::Error::last_os_error());
            }
            if libc::ftruncate(shm_fd, self.shm_pool_size as i64) == -1 {
                panic!("Couldn't truncate shm {}", std::io::Error::last_os_error());
            }
        }

        let shm_pool = unsafe {
            let prot = libc::PROT_READ | libc::PROT_WRITE;
            libc::mmap(
                std::ptr::null_mut(),
                self.shm_pool_size as usize,
                prot,
                libc::MAP_SHARED,
                shm_fd,
                0,
            )
        };

        if shm_pool.is_null() {
            panic!("Couldn't mmap shm {}", std::io::Error::last_os_error());
        }

        self.shm_fd = shm_fd;
        self.shm_data = shm_pool as *mut u8;

        self.draw(conn);

        let wl_shm = self.wl_shm.as_ref().unwrap();
        let wl_shm_pool = wl_shm.create_pool(conn, self.shm_fd, self.shm_pool_size);
        let wl_buffer = wl_shm_pool.create_buffer(conn, 0, self.width, self.height, self.stride, 1);

        self.callbacks.push((wl_buffer.id, Self::on_wlbuffer_event));
        self.wl_shm_pool = Some(wl_shm_pool);
        self.wl_buffer = Some(wl_buffer);
    }

    fn draw(&mut self, conn: &Connection) {
        let width = self.width as usize;
        let height = self.height as usize;

        let mut pixels = unsafe {
            std::slice::from_raw_parts_mut(
                self.shm_data as *mut u32,
                self.shm_pool_size as usize / 4,
            )
        };

        // pixels.fill(0x000000);
        for (y, row) in pixels.chunks_mut(width).enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let r = ((x * 255) / width) as u32;
                let g = 0;
                let b = ((y * 255) / height) as u32;
                *cell = (r << 16) | (g << 8) | b;
            }
        }

        // let dmgbox = rect {
        //     x: self.rect.x - self.dx,
        //     y: self.rect.y - self.dy,
        //     h: self.rect.h + (self.dy.abs() * 3),
        //     w: self.rect.w + (self.dx.abs() * 3),
        // };
        //
        // let max_x = self.rect.x + self.rect.w;
        // let max_y = self.rect.y + self.rect.h;
        //
        // if (max_x) >= self.width || self.rect.x <= 0 {
        //     self.dx *= -1;
        // }
        //
        // if (max_y) >= self.height || self.rect.y <= 0 {
        //     self.dy *= -1;
        // }
        //
        // self.rect.y = (self.rect.y + self.dy).clamp(0, self.height);
        // self.rect.x = (self.rect.x + self.dx).clamp(0, self.width);
        //
        // let max_x = self.rect.x + self.rect.w;
        // let max_y = self.rect.y + self.rect.h;
        //
        // // println!("rect: {:?}", self.rect);
        // for y in (self.rect.y..max_y) {
        //     for x in (self.rect.x..max_x) {
        //         // if let some(p) = pixels.get_mut((y*self.width + x) as usize) {
        //         //     *p = 0x0000ffff;
        //         // }
        //         pixels[(y * self.width + x) as usize] = 0x0000ffff;
        //     }
        // }

        // struct circle {
        //     x: i32,
        //     y: i32,
        //     r: i32,
        // }
        //
        // let circle = circle {
        //     x: 50,
        //     y: 50,
        //     r: 50,
        // };
        //
        // let begin_x = (circle.x - circle.r).min(width as i32);
        // let begin_y = (circle.y - circle.r).min(height as i32);
        //
        // let end_x = (circle.x + circle.r).min(width as i32);
        // let end_y = (circle.y + circle.r).min(height as i32);
        //
        // let r_seq = circle.r * circle.r;
        //
        // // let w = circle.r * 2;
        // // let h = w;
        //
        // for y in (begin_y..end_y) {
        //     for x in (begin_x..end_x) {
        //         let dx = (circle.x as i32 - x as i32);
        //         let dy = (circle.y as i32 - y as i32);
        //         if (dx * dx) + (dy * dy) < r_seq {
        //             // pixels[y * (state.width as usize) + x] = (r << 16) | (g << 8) | b;
        //             pixels[(y * width as i32 + x) as usize] = 0xff00ff;
        //         }
        //     }
        // }

        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            return; // this should never be reached
        };
        let Some(wl_surface) = self.wl_surface.as_ref() else {
            return;
        };

        wl_surface.attach(conn, Some(wl_buffer), 0, 0);
        // wl_surface.damage_buffer(conn, dmgbox.x, dmgbox.y, dmgbox.w, dmgbox.h);
        wl_surface.commit(conn);
    }

    fn cleanup(&self) {
        unsafe {
            if self.shm_fd != 0 {
                libc::close(self.shm_fd);
            }
            if !self.shm_data.is_null() {
                libc::munmap(
                    self.shm_data as *mut libc::c_void,
                    core::mem::size_of_val(self.shm_data.as_mut().unwrap()),
                );
            }
        }
    }
}
