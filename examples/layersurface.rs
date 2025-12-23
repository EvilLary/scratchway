#![allow(unused)]

use scratchway::connection::Connection;
use scratchway::events::Event;
use scratchway::protocols::core::*;
use scratchway::protocols::wlr_layer_shell_unstable_v1::*;
use scratchway::protocols::wp_single_pixel_buffer_manager_v1::*;

fn main() -> std::io::Result<()> {
    let conn = Connection::connect()?;
    let target_output = std::env::vars().skip(1).next();

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&conn);

    let mut callbacks: Vec<(u32, Callback)> = Vec::with_capacity(16);

    callbacks.push((wl_registry.id, State::on_registry_event));
    callbacks.push((wl_display.id, State::on_wldisplay_event));

    let mut state = State {
        wl_display,
        wl_registry: Some(wl_registry),
        callbacks,
        ..Default::default()
    };

    let events = conn.blocking_read();
    for event in events {
        state.handle_event(&conn, event);
    }

    if state.wlr_layer_shell.is_none() {
        eprintln!("Compositor doesn't support zwlr_layer_shell_v1 protocol");
        return Ok(());
    }

    conn.flush()?;

    state.init_layer(&conn);

    while !state.exit {
        let events = conn.blocking_read();
        for event in events {
            state.handle_event(&conn, event);
        }
    }

    Ok(())
}

type Callback = fn(&mut State, &Connection, Event<'_>);
#[derive(Debug, Default)]
struct State {
    callbacks: Vec<(u32, Callback)>,

    wl_display: WlDisplay,
    wl_registry: Option<WlRegistry>,
    wl_compositor: Option<WlCompositor>,
    wlr_layer_shell: Option<WlrLayerShell>,

    wl_surface: Option<WlSurface>,
    wl_buffer: Option<WlBuffer>,
    layer_surface: Option<WlrLayerSurface>,
    configured: bool,

    window_height: i32,
    window_width: i32,
    window_size_changed: bool,

    exit: bool,
}

impl State {
    fn init_layer(&mut self, conn: &Connection) {
        let Some(wlr_layer_shell) = self.wlr_layer_shell.as_ref() else {
            panic!("bruh");
        };
        let Some(wl_compositor) = self.wl_compositor.as_ref() else {
            panic!("bruh");
        };
        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            panic!("bruh");
        };

        let wl_surface = wl_compositor.create_surface(conn);
        self.callbacks
            .push((wl_surface.id, Self::on_wlsurface_event));

        let layer_surface = wlr_layer_shell.get_layer_surface(
            conn,
            &wl_surface,
            None,
            WlrLayer::Overlay,
            "crosshair",
        );
        self.callbacks
            .push((layer_surface.id, Self::on_layersurface_event));

        layer_surface.set_exclusive_zone(conn, 0);
        layer_surface.set_anchor(conn, 0);
        layer_surface.set_margin(conn, 0, 0, 0, 0);
        layer_surface.set_keyboard_interactivity(conn, WlrLayerKeyboard::None);
        layer_surface.set_size(conn, 5, 5);
        wl_surface.commit(conn);

        self.layer_surface = Some(layer_surface);
        self.wl_surface = Some(wl_surface);
    }

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
                "wl_compositor" => {
                    let wl_compositor: WlCompositor =
                        wl_registry.bind(&conn, name, interface, version);
                    self.wl_compositor = Some(wl_compositor);
                }
                "zwlr_layer_shell_v1" => {
                    let wlr_layer_shell: WlrLayerShell =
                        wl_registry.bind(&conn, name, interface, version);
                    self.wlr_layer_shell = Some(wlr_layer_shell);
                }
                "wp_single_pixel_buffer_manager_v1" => {
                    let spm: WpSinglePixelBufferMgr =
                        wl_registry.bind(&conn, name, interface, version);
                    let wl_buffer = spm.create_buffer(conn, (u32::MAX / 255) * 255, (u32::MAX / 255) * 0, (u32::MAX / 255) * 0, u32::MAX);
                    self.callbacks.push((wl_buffer.id, Self::on_wlbuffer_event));
                    self.wl_buffer = Some(wl_buffer);
                    spm.destroy(conn);
                }
                _ => {}
            },
            WlRegistryEvent::GlobalRemove { name } => {
                println!("Removed: {:?}", name);
            }
        }
    }

    fn on_layersurface_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(layer_surface) = self.layer_surface.as_ref() else {
            return;
        };
        match layer_surface.parse_event(event) {
            WlrLayerSurfaceEvent::Configure {
                serial,
                width,
                height,
            } => {
                layer_surface.ack_configure(conn, serial);
                if !self.configured {
                    let Some(wl_surface) = self.wl_surface.as_ref() else {
                        return;
                    };
                    layer_surface.set_size(conn, width, height);
                    wl_surface.attach(conn, self.wl_buffer.as_ref(), 0, 0);
                    wl_surface.commit(conn);
                    self.configured = true;
                }
            }
            WlrLayerSurfaceEvent::Closed => {
                self.exit = true;
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

    fn on_wlbuffer_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            return; // this should never be reached
        };
        match wl_buffer.parse_event(event) {
            WlBufferEvent::Release => {
            }
        }
    }
}
