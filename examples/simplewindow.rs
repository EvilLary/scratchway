#![allow(unused)]

use std::{
    ffi::c_str,
    os::fd::{AsRawFd, RawFd},
};

use scratchway::log;
use scratchway::prelude::*;
use scratchway::wayland::*;

use scr_protocols::{
    viewporter::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
    xdg_shell::{xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase, *},
};

use clibs::cairo;
use clibs::xkbcommon;

mod clibs;

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&mut conn);

    conn.add_callback(&wl_registry, App::on_wlregistry);
    conn.add_callback(&wl_display, App::on_wldisplay);

    let mut state = App {
        wl_display,
        wl_registry: Some(wl_registry),
        ..Default::default()
    };

    conn.roundtrip(&mut state)?;
    while !state.exit {
        conn.blocking_dispatch(&mut state)?;
    }

    Ok(())
}

#[derive(Debug, Default)]
struct App {
    wl_display: wl_display::WlDisplay,
    wl_registry: Option<wl_registry::WlRegistry>,
    wl_seat: Option<wl_seat::WlSeat>,
    wl_compositor: Option<wl_compositor::WlCompositor>,
    xdg_wm_base: Option<XdgWmBase>,
    viewporter: Option<WpViewporter>,

    wl_surface: Option<wl_surface::WlSurface>,
    wl_buffer: Option<wl_buffer::WlBuffer>,
    xdg_toplevel: Option<XdgToplevel>,
    xdg_surface: Option<XdgSurface>,
    viewport: Option<WpViewport>,
    configured: bool,

    wl_pointer: Option<wl_pointer::WlPointer>,
    wl_keyboard: Option<wl_keyboard::WlKeyboard>,
    xkb: Xkb,

    wl_shm: Option<wl_shm::WlShm>,
    wl_shm_pool: Option<wl_shm_pool::WlShmPool>,
    shm_fd: RawFd,
    shm_data: *mut u8,
    shm_pool_size: i32,
    width: i32,
    height: i32,
    stride: i32,

    window_height: i32,
    window_width: i32,
    window_size_changed: bool,

    exit: bool,
}

#[derive(Debug, Default)]
struct Xkb {
    ctx: *mut xkbcommon::xkb_context,
    keymap: *mut xkbcommon::xkb_keymap,
    state: *mut xkbcommon::xkb_state,
}

impl App {
    fn on_wlseat(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let wl_seat = unsafe { self.wl_seat.as_ref().unwrap_unchecked() };
        match wl_seat.parse_event(conn, &event) {
            wl_seat::Event::Capabilities { capabilities } => {
                if capabilities & wl_seat::CAPABILITY_POINTER > 0 {
                    let wl_pointer = wl_seat.get_pointer(conn);
                    wl_pointer.set_callback(
                        conn,
                        |state: &mut Self, conn: &mut Connection<Self>, event: WlEvent| {
                            let wl_pointer = unsafe { state.wl_pointer.as_ref().unwrap_unchecked() };
                            match wl_pointer.parse_event(conn, &event) {
                                _ => {},
                            }
                        },
                    );
                    self.wl_pointer = Some(wl_pointer);
                }
                if capabilities & wl_seat::CAPABILITY_KEYBOARD > 0 {
                    let wl_keyboard = wl_seat.get_keyboard(conn);
                    conn.add_callback(&wl_keyboard, Self::on_wlkeyboard);
                    self.wl_keyboard = Some(wl_keyboard);
                }
            },
            wl_seat::Event::Name { .. } => {},
        }
    }

    fn on_wlkeyboard(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let wl_keyboard = unsafe { self.wl_keyboard.as_ref().unwrap_unchecked() };
        match wl_keyboard.parse_event(conn, &event) {
            wl_keyboard::Event::Keymap { fd, size, .. } => unsafe {
                let p_keymap = libc::mmap(
                    core::ptr::null_mut(),
                    size as usize,
                    libc::PROT_READ,
                    libc::MAP_PRIVATE,
                    fd.as_raw_fd(),
                    0,
                );
                assert!(!p_keymap.is_null());
                let xkb_ctx = xkbcommon::xkb_context_new(xkbcommon::XKB_CONTEXT_NO_FLAGS);
                assert!(!xkb_ctx.is_null());
                let xkb_keymap = xkbcommon::xkb_keymap_new_from_buffer(
                    xkb_ctx,
                    p_keymap.cast(),
                    size as usize,
                    xkbcommon::XKB_KEYMAP_FORMAT_TEXT_V1,
                    xkbcommon::XKB_KEYMAP_COMPILE_NO_FLAGS,
                );
                assert!(!xkb_keymap.is_null());
                let state = xkbcommon::xkb_state_new(xkb_keymap);
                assert!(!state.is_null());
                self.xkb = Xkb {
                    keymap: xkb_keymap,
                    ctx: xkb_ctx,
                    state,
                };
                libc::munmap(p_keymap, core::mem::size_of_val(p_keymap.as_mut().unwrap()));
            },
            wl_keyboard::Event::Key { key, state, .. } => {
                if !self.xkb.ctx.is_null() {
                    unsafe {
                        // let mut buf = [0u8; 5];
                        let keysym = xkbcommon::xkb_state_key_get_one_sym(self.xkb.state, key + 8);
                        // let size = xkbcommon::xkb_state_key_get_utf8(
                        //     self.xkb.state,
                        //     key + 8,
                        //     buf.as_mut_ptr().cast(),
                        //     4,
                        // );
                        if state == 1 {
                            let mut name = [0u8; 64];
                            let len =
                                xkbcommon::xkb_keysym_get_name(keysym, name.as_mut_ptr().cast(), name.len());
                            self.draw(
                                conn,
                                c_str::CStr::from_bytes_with_nul(&name[..1 + len as usize]).unwrap(),
                            );
                        }
                        xkbcommon::xkb_state_update_key(self.xkb.state, key + 8, state);
                        // log!(
                        //     DEBUG,
                        //     "{:?}",
                        //     core::str::from_utf8_unchecked(&buf[..size as usize])
                        // );
                    }
                }
            },
            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => unsafe {
                xkbcommon::xkb_state_update_mask(
                    self.xkb.state,
                    mods_depressed,
                    mods_latched,
                    mods_locked,
                    group,
                    0,
                    0,
                );
            },
            _ => {},
        }
    }

    fn on_wldisplay(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        match self.wl_display.parse_event(conn, &event) {
            wl_display::Event::Error {
                object_id,
                code,
                message,
            } => {
                eprintln!("Protocol error: code {code} from object {object_id}, {message}");
                self.exit = true;
            },
            wl_display::Event::DeleteId { id } => {},
        }
    }

    fn on_wlregistry(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let wl_registry = unsafe { self.wl_registry.as_ref().unwrap_unchecked() };
        match wl_registry.parse_event(conn, &event) {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => match interface {
                "wp_viewporter" => {
                    self.viewporter = Some(wl_registry.bind(conn, name, interface, version));
                },
                "wl_shm" => {
                    let wl_shm: wl_shm::WlShm = wl_registry.bind(conn, name, interface, version);
                    self.wl_shm = Some(wl_shm);
                    self.init_shm(conn);
                },
                "wl_seat" => {
                    let wl_seat: wl_seat::WlSeat = wl_registry.bind(conn, name, interface, version);
                    conn.add_callback(&wl_seat, Self::on_wlseat);
                    self.wl_seat = Some(wl_seat);
                },
                "wl_compositor" => {
                    let wl_compositor: wl_compositor::WlCompositor =
                        wl_registry.bind(conn, name, interface, version);
                    let wl_surface = wl_compositor.create_surface(conn);

                    conn.add_callback(&wl_surface, Self::on_wlsurface);
                    self.wl_surface = Some(wl_surface);
                },
                "xdg_wm_base" => {
                    let xdg_wm_base: XdgWmBase = wl_registry.bind(conn, name, interface, version);
                    conn.add_callback(&xdg_wm_base, Self::on_xdgwmbase);

                    self.xdg_wm_base = Some(xdg_wm_base);
                    if self.xdg_wm_base.is_some() && self.xdg_surface.is_none() {
                        self.init_toplevel(conn);
                    }
                },
                _ => {},
            },
            wl_registry::Event::GlobalRemove { .. } => {},
        }
    }

    fn on_xdgsurface(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let xdg_surface = unsafe { self.xdg_surface.as_ref().unwrap_unchecked() };
        // let Some(xdg_surface) = self.xdg_surface.as_ref() else {
        //     return;
        // };
        match xdg_surface.parse_event(conn, &event) {
            xdg_surface::Event::Configure { serial } => {
                xdg_surface.ack_configure(conn, serial);

                let wl_surface = self.wl_surface.as_ref().unwrap();
                if let Some(ref wl_buffer) = self.wl_buffer
                    && !self.configured
                {
                    wl_surface.set_input_region(conn, None);
                    wl_surface.attach(conn, Some(wl_buffer), 0, 0);
                    wl_surface.commit(conn);
                    self.configured = true;
                }

                if self.window_size_changed {
                    xdg_surface.set_window_geometry(conn, 0, 0, self.window_width, self.window_height);
                    if let Some(ref viewport) = self.viewport {
                        viewport.set_destination(conn, self.window_width, self.window_height);
                    }
                    wl_surface.damage_buffer(conn, 0, 0, self.width, self.height);
                    wl_surface.commit(conn);
                    self.window_size_changed = false;
                }
            },
        }
    }

    fn on_wlsurface(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let Some(wl_surface) = self.wl_surface.as_ref() else {
            return;
        };
        match wl_surface.parse_event(conn, &event) {
            _ => {},
        }
    }

    fn on_xdgwmbase(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let xdg_wm_base = unsafe { self.xdg_wm_base.as_ref().unwrap_unchecked() };
        match xdg_wm_base.parse_event(conn, &event) {
            xdg_wm_base::Event::Ping { serial } => {
                xdg_wm_base.pong(conn, serial);
            },
        }
    }

    fn on_xdgtoplevel(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let xdg_toplevel = unsafe { self.xdg_toplevel.as_ref().unwrap_unchecked() };
        match xdg_toplevel.parse_event(conn, &event) {
            xdg_toplevel::Event::Configure { width, height, .. } => {
                if width != 0 && height != 0 {
                    if self.window_width != width || self.window_height != height {
                        self.window_height = height;
                        self.window_width = width;
                        self.window_size_changed = true;
                    }
                }
            },
            xdg_toplevel::Event::Close => {
                self.exit = true;
                self.cleanup(conn);
            },
            _ => {},
        }
    }

    fn on_wlbuffer(&mut self, conn: &mut Connection<Self>, event: WlEvent) {
        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            return;
        };
        match wl_buffer.parse_event(conn, &event) {
            wl_buffer::Event::Release => {},
        }
    }

    fn init_toplevel(&mut self, conn: &mut Connection<Self>) {
        let wl_surface = unsafe { self.wl_surface.as_ref().unwrap_unchecked() };
        let xdg_wm_base = unsafe { self.xdg_wm_base.as_ref().unwrap_unchecked() };

        let xdg_surface = xdg_wm_base.get_xdg_surface(conn, wl_surface);
        let xdg_toplevel = xdg_surface.get_toplevel(conn);

        xdg_toplevel.set_title(conn, "Hola bola");
        xdg_toplevel.set_app_id(conn, "com.github.evillary");
        xdg_toplevel.set_max_size(conn, 400, 400);
        xdg_toplevel.set_min_size(conn, 400, 400);

        if let Some(ref viewporter) = self.viewporter {
            let viewport = viewporter.get_viewport(conn, wl_surface);
            viewport.set_destination(conn, self.width, self.height);
            self.viewport = Some(viewport);
        }

        xdg_surface.set_window_geometry(conn, 0, 0, self.width, self.height);
        wl_surface.commit(conn);

        conn.add_callback(&xdg_surface, Self::on_xdgsurface);
        conn.add_callback(&xdg_toplevel, Self::on_xdgtoplevel);

        self.xdg_toplevel = Some(xdg_toplevel);
        self.xdg_surface = Some(xdg_surface);
    }

    fn init_shm(&mut self, conn: &mut Connection<Self>) {
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

        self.draw(conn, c"Press anything");

        let wl_shm = self.wl_shm.as_ref().unwrap();
        let wl_shm_pool = wl_shm.create_pool(conn, self.shm_fd, self.shm_pool_size);
        let wl_buffer = wl_shm_pool.create_buffer(conn, 0, self.width, self.height, self.stride, 1);
        // unsafe {
        //     libc::close(self.shm_fd);
        //     self.shm_fd = 0;
        // }
        // wl_shm_pool.destroy(conn);

        conn.add_callback(&wl_buffer, Self::on_wlbuffer);
        self.wl_shm_pool = Some(wl_shm_pool);
        self.wl_buffer = Some(wl_buffer);
    }

    fn draw(&mut self, conn: &mut Connection<Self>, text: &c_str::CStr) {
        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            return; // this should never be reached
        };

        let Some(wl_surface) = self.wl_surface.as_ref() else {
            return;
        };

        unsafe {
            let surface = {
                let format = cairo::CAIRO_FORMAT_ARGB32;
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
            cairo::cairo_set_source_rgb(cr, 75.0 / 255.0, 109.0 / 255.0, 145.0 / 255.0);
            cairo::cairo_rectangle(cr, 10.0, 10.0, (self.width - 20) as _, (self.height - 20) as _);

            cairo::cairo_fill(cr);
            // cairo::cairo_stroke(cr);

            // let text = c"هلا".as_ptr();
            let text = text.as_ptr();
            let font = c"Noto Sans".as_ptr();
            cairo::cairo_set_source_rgb(cr, 1.0, 1.0, 1.0);
            cairo::cairo_select_font_face(
                cr,
                font,
                cairo::CAIRO_FONT_SLANT_NORMAL,
                cairo::CAIRO_FONT_WEIGHT_BOLD,
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

        wl_surface.attach(conn, Some(wl_buffer), 0, 0);
        wl_surface.damage_buffer(conn, 0, 0, self.width, self.height);
        wl_surface.commit(conn);
    }

    fn cleanup(&self, conn: &mut Connection<Self>) {
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
            // if !self.xkb_keymap.is_null() {
            //     libc::munmap(
            //         self.xkb_keymap,
            //         core::mem::size_of_val(self.xkb_keymap.as_mut().unwrap()),
            //     );
            // }
        }

        if let Some(ref o) = self.wl_pointer {
            o.release(conn);
        }

        if let Some(ref o) = self.wl_buffer {
            o.destroy(conn);
        }

        if let Some(ref o) = self.xdg_toplevel {
            o.destroy(conn);
        }

        if let Some(ref o) = self.viewport {
            o.destroy(conn);
        }

        if let Some(ref o) = self.viewporter {
            o.destroy(conn);
        }

        if let Some(ref o) = self.xdg_surface {
            o.destroy(conn);
        }

        if let Some(ref o) = self.xdg_wm_base {
            o.destroy(conn);
        }

        if let Some(ref o) = self.wl_surface {
            o.destroy(conn);
        }
    }
}
