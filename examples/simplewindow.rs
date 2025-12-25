#![allow(unused)]

use scratchway::connection::Connection;
use scratchway::events::Event;
use scratchway::protocols::core::*;
use scratchway::protocols::viewporter::*;
use scratchway::protocols::xdg_shell::*;
use std::os::fd::RawFd;

use clibs::cairo;
mod clibs;

fn main() -> std::io::Result<()> {
    let conn = Connection::connect()?;

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&conn);

    let mut callbacks: Vec<(u32, Callback)> = Vec::with_capacity(16);

    callbacks.push((wl_registry.id(), State::on_wlregistry));
    callbacks.push((wl_display.id(), State::on_wldisplay));

    let mut state = State {
        wl_display,
        wl_registry: Some(wl_registry),
        callbacks,
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

    wl_display:    WlDisplay,
    wl_registry:   Option<WlRegistry>,
    wl_seat:       Option<WlSeat>,
    wl_compositor: Option<WlRegistry>,
    xdg_wm_base:   Option<XdgWmBase>,
    viewporter:    Option<WpViewporter>,

    wl_surface:   Option<WlSurface>,
    wl_buffer:    Option<WlBuffer>,
    xdg_toplevel: Option<XdgToplevel>,
    xdg_surface:  Option<XdgSurface>,
    viewport:     Option<WpViewport>,
    configured:   bool,

    wl_pointer: Option<WlPointer>,

    wl_shm:        Option<WlShm>,
    wl_shm_pool:   Option<WlShmPool>,
    shm_fd:        RawFd,
    shm_data:      *mut u8,
    shm_pool_size: i32,
    width:         i32,
    height:        i32,
    stride:        i32,

    window_height:       i32,
    window_width:        i32,
    window_size_changed: bool,

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

    fn on_wlseat(&mut self, conn: &Connection, event: Event) {
        let wl_seat = unsafe { self.wl_seat.as_ref().unwrap_unchecked() };
        match wl_seat.parse_event(event) {
            WlSeatEvent::Capabilities {
                capabilities,
            } => {
                if capabilities & 1 > 0 {
                    let wl_pointer = wl_seat.get_pointer(conn);
                    self.callbacks.push((wl_pointer.id, Self::on_wlpointer));
                    self.wl_pointer = Some(wl_pointer);
                }
            }
            WlSeatEvent::Name {
                name,
            } => {},
        }
    }

    fn on_wlpointer(&mut self, _conn: &Connection, event: Event) {
        let wl_pointer = unsafe { self.wl_pointer.as_ref().unwrap_unchecked() };
        match wl_pointer.parse_event(event) {
            WlPointerEvent::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
            } => {
                println!("Whoooo")
            }
            WlPointerEvent::Leave {
                serial,
                surface,
            } => {},
            WlPointerEvent::Motion {
                time,
                surface_x,
                surface_y,
            } => {}
            WlPointerEvent::Button {
                serial,
                time,
                button,
                state,
            } => {
                println!("{:?}", button);
            }
            WlPointerEvent::Axis { time, axis, value } => {},
            WlPointerEvent::Frame => {},
        }
    }
    fn on_wldisplay(&mut self, _conn: &Connection, event: Event) {
        match self.wl_display.parse_event(event) {
            WlDisplayEvent::Error {
                object_id,
                code,
                message,
            } => {
                eprintln!("Protocol error: code {code} from object {object_id}, {message}");
                self.exit = true;
            }
            WlDisplayEvent::DeleteId {
                id,
            } => self.callbacks.retain(|(obj_id, _)| id != *obj_id),
        }
    }

    fn on_wlregistry(&mut self, conn: &Connection, event: Event) {
        let wl_registry = unsafe { self.wl_registry.as_ref().unwrap_unchecked() };
        // let Some(wl_registry) = self.wl_registry.as_ref() else {
        //     return; // this should never be reached
        // };
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
                "wl_seat" => {
                    let wl_seat: WlSeat = wl_registry.bind(&conn, name, interface, version);
                    self.callbacks.push((wl_seat.id, Self::on_wlseat));
                    self.wl_seat = Some(wl_seat);
                }
                "wl_compositor" => {
                    let wl_compositor: WlCompositor =
                        wl_registry.bind(&conn, name, interface, version);
                    let wl_surface = wl_compositor.create_surface(conn);

                    self.callbacks.push((wl_surface.id, Self::on_wlsurface));
                    self.wl_surface = Some(wl_surface);
                }
                "xdg_wm_base" => {
                    let xdg_wm_base: XdgWmBase = wl_registry.bind(&conn, name, interface, version);
                    self.callbacks.push((xdg_wm_base.id, Self::on_xdgwmbase));

                    self.xdg_wm_base = Some(xdg_wm_base);
                    if self.xdg_wm_base.is_some() && self.xdg_surface.is_none() {
                        self.init_toplevel(conn);
                    }
                }
                _ => {}
            },
            WlRegistryEvent::GlobalRemove {
                name,
            } => {
                println!("Removed: {:?}", name);
            }
        }
    }

    fn on_xdgsurface(&mut self, conn: &Connection, event: Event<'_>) {
        let xdg_surface = unsafe { self.xdg_surface.as_ref().unwrap_unchecked() };
        // let Some(xdg_surface) = self.xdg_surface.as_ref() else {
        //     return;
        // };
        match xdg_surface.parse_event(event) {
            XdgSurfaceEvent::Configure {
                serial,
            } => {
                xdg_surface.ack_configure(conn, serial);

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
            }
        }
    }

    fn on_wlsurface(&mut self, conn: &Connection, event: Event<'_>) {
        // let Some(wl_surface) = self.wl_surface.as_ref() else {
        //     return;
        // };
        // match wl_surface.parse_event(event) {
        //     WlSurfaceEvent::Enter { output } => {}
        //     WlSurfaceEvent::Leave { output } => {}
        //     WlSurfaceEvent::PrefferedBufferScale { factor } => {}
        //     WlSurfaceEvent::PrefferedBufferTransform { transform } => {}
        // }
    }

    fn on_xdgwmbase(&mut self, conn: &Connection, event: Event<'_>) {
        let xdg_wm_base = unsafe { self.xdg_wm_base.as_ref().unwrap_unchecked() };
        // let Some(xdg_wm_base) = self.xdg_wm_base.as_ref() else {
        //     return;
        // };
        match xdg_wm_base.parse_event(event) {
            XdgWmBaseEvent::Ping {
                serial,
            } => {
                xdg_wm_base.pong(conn, serial);
                // self.draw(conn);
            }
        }
    }

    fn on_xdgtoplevel(&mut self, conn: &Connection, event: Event<'_>) {
        let xdg_toplevel = unsafe { self.xdg_toplevel.as_ref().unwrap_unchecked() };
        // let Some(xdg_toplevel) = self.xdg_toplevel.as_ref() else {
        //     return;
        // };
        match xdg_toplevel.parse_event(event) {
            XdgToplevelEvent::Configure {
                width,
                height,
                ..
            } => {
                if width != 0 && height != 0 {
                    if self.window_width != width || self.window_height != height {
                        self.window_height = height;
                        self.window_width = width;
                        self.window_size_changed = true;
                    }
                }
            }
            // XdgToplevelEvent::ConfigureBounds { width, height } => {}
            XdgToplevelEvent::Close => self.exit = true,
            // XdgToplevelEvent::WmCapabilities { capabilities } => {}
            _ => {}
        }
    }

    fn on_wlbuffer(&mut self, conn: &Connection, event: Event<'_>) {
        // let Some(wl_buffer) = self.wl_buffer.as_ref() else {
        //     return; // this should never be reached
        // };
        // match wl_buffer.parse_event(event) {
        //     WlBufferEvent::Release => {
        //         // self.draw(conn);
        //         // std::thread::sleep(std::time::Duration::from_millis(1000));
        //     }
        // }
    }

    fn init_toplevel(&mut self, conn: &Connection) {
        // let Some(wl_surface) = self.wl_surface.as_ref() else {
        //     return;
        // };
        // let Some(xdg_wm_base) = self.xdg_wm_base.as_ref() else {
        //     return;
        // };

        let wl_surface = unsafe { self.wl_surface.as_ref().unwrap_unchecked() };
        let xdg_wm_base = unsafe { self.xdg_wm_base.as_ref().unwrap_unchecked() };

        let xdg_surface = xdg_wm_base.get_xdg_surface(conn, wl_surface);
        self.callbacks.push((xdg_surface.id, Self::on_xdgsurface));

        let xdg_toplevel = xdg_surface.get_toplevel(conn);
        self.callbacks.push((xdg_toplevel.id, Self::on_xdgtoplevel));

        xdg_toplevel.set_title(conn, "Hola bola");
        xdg_toplevel.set_app_id(conn, "com.github.evillary");

        if let Some(ref viewporter) = self.viewporter {
            let viewport = viewporter.get_viewport(conn, wl_surface);
            viewport.set_destination(conn, self.width, self.height);
            self.viewport = Some(viewport);
        }

        // wl_surface.set_buffer_scale(conn, 2);
        xdg_surface.set_window_geometry(conn, 0, 0, self.width, self.height);
        // wl_surface.attach(conn, self.wl_buffer.as_ref(), 0, 0);
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
            libc::shm_open(name, flags, 0o600)
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
        // wl_shm_pool.destroy(conn);

        self.callbacks.push((wl_buffer.id, Self::on_wlbuffer));
        self.wl_shm_pool = Some(wl_shm_pool);
        self.wl_buffer = Some(wl_buffer);
    }

    fn draw(&mut self, conn: &Connection) {
        // let mut pixels = unsafe {
        //     std::slice::from_raw_parts_mut(
        //         self.shm_data as *mut u32,
        //         self.shm_pool_size as usize / 4,
        //     )
        // };
        unsafe {
            let surface = {
                let format = cairo::CairoFormat::ARGB32;
                cairo::cairo_image_surface_create_for_data(
                    self.shm_data,
                    format as i32,
                    self.width,
                    self.height,
                    self.stride,
                )
            };
            assert!(!surface.is_null());

            let cr = cairo::cairo_create(surface);
            assert!(!cr.is_null());

            let mut te: cairo::cairo_text_extents_t = std::mem::zeroed();

            // cairo::cairo_set_line_width(cr, 5.0);
            // cairo::cairo_set_source_rgb(cr, 1.0, 0.0, 0.0);
            // cairo::cairo_move_to(cr, 0.0, 0.0);
            // cairo::cairo_line_to(cr, 250.0, 250.0);
            // cairo::cairo_rel_line_to(cr, -150.0, 150.0);
            // cairo::cairo_stroke(cr);

            cairo::cairo_set_line_width(cr, 5.0);
            cairo::cairo_set_source_rgb(cr, 0.6, 0.5, 0.3);
            cairo::cairo_rectangle(
                cr,
                10.0,
                10.0,
                (self.width - 20) as _,
                (self.height - 20) as _,
            );

            // cairo::cairo_fill(cr);
            cairo::cairo_stroke(cr);

            let text = c"Hola Bola".as_ptr();
            let font = c"Noto Sans".as_ptr();
            cairo::cairo_set_source_rgb(cr, 0.3, 0.5, 0.6);
            cairo::cairo_select_font_face(
                cr,
                font,
                cairo::CairoFontSlant::Normal as i32,
                cairo::CairoFontWeight::Normal as i32,
            );
            cairo::cairo_set_line_width(cr, 10.0);
            cairo::cairo_set_font_size(cr, 25.0);
            cairo::cairo_text_extents(cr, text, &mut te as *mut _);
            cairo::cairo_move_to(
                cr,
                (self.width as f64 / 2.0) - (te.width / 2.0),
                (self.height as f64 / 2.0) - (te.height / 2.0),
            );

            cairo::cairo_show_text(cr, text);

            cairo::cairo_surface_destroy(surface);
            cairo::cairo_destroy(cr);
        }

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
