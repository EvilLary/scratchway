#![allow(unused)]

use scratchway::connection::Connection;
use scratchway::connection::State;
use scratchway::events::Event;
use scratchway::protocols::core::*;
use scratchway::protocols::viewporter::*;
use scratchway::protocols::wlr_layer_shell_unstable_v1::*;
use scratchway::protocols::wp_single_pixel_buffer_manager_v1::*;

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;
    let target_output = std::env::args().skip(1).next();

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&conn);

    let mut callbacks: Vec<(u32, Callback)> = vec![
        (wl_registry.id, WaylandState::on_registry_event),
        (wl_display.id, WaylandState::on_wldisplay_event),
    ];

    let mut state = WaylandState {
        wl_display,
        wl_registry: Some(wl_registry),
        callbacks,
        ..Default::default()
    };

    conn.roundtrip(&mut state);

    if state.wlr_layer_shell.is_none() {
        eprintln!("Compositor doesn't support zwlr_layer_shell_v1 protocol");
        std::process::exit(1);
    }

    if state.wl_buffer.is_none() {
        eprintln!("Compositor doesn't support wp_single_pixel_buffer_manager_v1 protocol");
        std::process::exit(1);
    }

    conn.roundtrip(&mut state);

    state.init_layer(&conn, target_output);

    while !state.exit {
        conn.dispatch_events(&mut state);
    }

    Ok(())
}

#[derive(Debug)]
struct Output {
    wl_output: WlOutput,
    port:      String,
    name:      u32,
    width:     i32,
    height:    i32,
    ready:     bool,
}

type Callback = fn(&mut WaylandState, &Connection, Event<'_>);
#[derive(Debug, Default)]
struct WaylandState {
    callbacks: Vec<(u32, Callback)>,

    wl_display:      WlDisplay,
    wl_registry:     Option<WlRegistry>,
    wl_compositor:   Option<WlCompositor>,
    viewporter:      Option<WpViewporter>,
    outputs:         Vec<Output>,
    wlr_layer_shell: Option<WlrLayerShell>,

    wl_surface:    Option<WlSurface>,
    viewport:      Option<WpViewport>,
    wl_buffer:     Option<WlBuffer>,
    layer_surface: Option<WlrLayerSurface>,
    configured:    bool,

    window_height:       i32,
    window_width:        i32,
    window_size_changed: bool,

    exit: bool,
}

impl State for WaylandState {
    fn handle_event(&mut self, conn: &Connection, event: Event<'_>) {
        if let Some((_, cb)) = self.callbacks.iter().find(|(id, _)| *id == event.header.id) {
            cb(self, conn, event)
        } else {
            eprintln!(
                "[\x1b[33mWARNING\x1b[0m]: Unhandled event for id: {}, opcode: {}",
                event.header.id, event.header.opcode
            )
        }
    }
}

impl WaylandState {
    fn init_layer(&mut self, conn: &Connection, target: Option<String>) {
        let Some(wl_compositor) = self.wl_compositor.as_ref() else {
            panic!("How did this happen");
        };

        let wl_surface = wl_compositor.create_surface(conn);
        self.register_cb(Self::on_wlsurface_event, wl_surface.id);

        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            panic!("How did this happen");
        };

        let Some(wlr_layer_shell) = self.wlr_layer_shell.as_ref() else {
            panic!("How did this happen");
        };

        let Some(output) = self
            .outputs
            .iter()
            .filter(|o| {
                if !o.ready {
                    false
                } else if let Some(name) = target.as_ref() {
                    *name == o.port
                } else {
                    true
                }
            })
            .next()
        else {
            eprintln!("Couldn't find an output");
            std::process::exit(1);
        };

        let layer_surface = wlr_layer_shell.get_layer_surface(
            conn,
            &wl_surface,
            Some(&output.wl_output),
            WlrLayer::Top,
            "crosshair",
        );
        self.register_cb(Self::on_layersurface_event, layer_surface.id);

        if let Some(ref viewporter) = self.viewporter {
            let viewport = viewporter.get_viewport(conn, &wl_surface);
            viewport.set_destination(conn, 500, 100);
            self.viewport = Some(viewport);
        }

        let anchor = WlrLayerSurface::ANCHOR_TOP
            | WlrLayerSurface::ANCHOR_LEFT
            | WlrLayerSurface::ANCHOR_RIGHT
            | WlrLayerSurface::ANCHOR_BOTTOM;

        layer_surface.set_keyboard_interactivity(conn, WlrLayerKeyboard::None);
        layer_surface.set_exclusive_zone(conn, 0);
        layer_surface.set_anchor(conn, anchor);
        layer_surface.set_margin(conn, 0, 0, 0, 0);
        layer_surface.set_size(conn, 30, 30);

        wl_surface.commit(conn);

        self.layer_surface = Some(layer_surface);
        self.wl_surface = Some(wl_surface);
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
            WlDisplayEvent::DeleteId {
                id,
            } => self.callbacks.retain(|(obj_id, _)| id != *obj_id),
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
                    let wl_compositor = wl_registry.bind(&conn, name, interface, version);
                    self.wl_compositor = Some(wl_compositor);
                }
                "wp_viewporter" => {
                    let viewporter = wl_registry.bind(&conn, name, interface, version);
                    self.viewporter = Some(viewporter);
                }
                "zwlr_layer_shell_v1" => {
                    let wlr_layer_shell = wl_registry.bind(&conn, name, interface, version);
                    self.wlr_layer_shell = Some(wlr_layer_shell);
                }
                "wp_single_pixel_buffer_manager_v1" => {
                    let spm: WpSinglePixelBufferMgr =
                        wl_registry.bind(&conn, name, interface, version);
                    let wl_buffer = spm.create_buffer(
                        conn,
                        (u32::MAX / 255) * 71,
                        (u32::MAX / 255) * 150,
                        (u32::MAX / 255) * 220,
                        (u32::MAX / 255) * ((50 * 255) / 100),
                    );
                    self.register_cb(Self::on_wlbuffer_event, wl_buffer.id);
                    self.wl_buffer = Some(wl_buffer);
                    spm.destroy(conn);
                }
                "wl_output" => {
                    let wl_output: WlOutput = wl_registry.bind(&conn, name, interface, version);
                    self.register_cb(Self::on_output_event, wl_output.id);
                    self.outputs.push(Output {
                        wl_output,
                        port: String::new(),
                        name,
                        width: 0,
                        height: 0,
                        ready: false,
                    });
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
                if let Some(ref viewport) = self.viewport {
                    viewport.set_destination(conn, width as i32, height as i32);
                }
                layer_surface.ack_configure(conn, serial);
                if !self.configured {
                    let Some(wl_surface) = self.wl_surface.as_ref() else {
                        unreachable!();
                        return;
                    };
                    layer_surface.set_size(conn, width, height);
                    conn.flush();
                    wl_surface.attach(conn, self.wl_buffer.as_ref(), 0, 0);
                    // wl_surface.damage_buffer(conn, 0, 0, 500, 100);
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
            WlSurfaceEvent::Enter {
                output,
            } => {}
            WlSurfaceEvent::Leave {
                output,
            } => {}
            WlSurfaceEvent::PrefferedBufferScale {
                factor,
            } => {}
            WlSurfaceEvent::PrefferedBufferTransform {
                transform,
            } => {}
        }
    }

    fn on_wlbuffer_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(wl_buffer) = self.wl_buffer.as_ref() else {
            return; // this should never be reached
        };
        match wl_buffer.parse_event(event) {
            WlBufferEvent::Release => {}
        }
    }

    fn on_output_event(&mut self, conn: &Connection, event: Event<'_>) {
        let Some(output) = self
            .outputs
            .iter_mut()
            .find(|o| o.wl_output.id == event.header.id)
        else {
            return;
        };
        match output.wl_output.parse_event(event) {
            WlOutputEvent::Name {
                name,
            } => {
                output.port = name.into();
            }
            WlOutputEvent::Mode {
                width,
                height,
                ..
            } => {
                output.width = width;
                output.height = height;
            }
            WlOutputEvent::Done => output.ready = true,
            WlOutputEvent::Geometry {
                ..
            } => {}
            _ => {}
        }
    }

    fn register_cb(&mut self, cb: Callback, id: u32) {
        self.callbacks.push((id, cb));
    }
}
