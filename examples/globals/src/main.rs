#![allow(unused)]

use std::io::Read;

use scratchway::connection::Connection;
use scratchway::core::*;
use scratchway::events::{Event, EventDataParser, EventIter};

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;
    let mut state = State::init(&conn).unwrap();
    loop {
        let mut events = conn.blocking_read();
        while let Some(event) = events.next() {
            state.handle_event(&conn, event);
        }
    }
    // println!("Hello, world!");
    Ok(())
}

struct State {
    wl_display: WlDisplay,
    wl_registry: WlRegistry,
    wl_outputs: Vec<WlOutput>,
    wl_compositor: WlCompositor,

    wl_surface: WlSurface,
}

impl State {
    fn init(conn: &Connection) -> Option<Self> {
        let wl_display = conn.display();
        let wl_registry = wl_display.get_registry(&conn);
        let mut wl_compositor: Option<WlCompositor> = None;
        conn.blocking_read()
            .map(|ev| wl_registry.parse_event(ev))
            .filter_map(|global| {
                if let WlRegistryEvent::Global {
                    name,
                    interface,
                    version,
                } = global
                {
                    if (interface) == "wl_compositor" {
                        Some((name, interface, version))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .for_each(|g| match g.1 {
                "wl_compositor" => wl_compositor = Some(wl_registry.bind(conn, g.0, g.1, g.2)),
                _ => {}
            });

        let wl_compositor = wl_compositor?;
        let wl_outputs = Vec::new();
        conn.flush().unwrap();

        let wl_surface = wl_compositor.create_surface(conn);
        Some(Self {
            wl_compositor,
            wl_registry,
            wl_outputs,
            wl_display,
            wl_surface,
        })
    }
    fn handle_event(&mut self, conn: &Connection, event: Event) {
        match event.header.id {
            id if id == self.wl_display.id => {
                let ev = self.wl_display.parse_event(event);
                println!("{:?}", ev);
            }
            id if id == self.wl_registry.id => self.handle_reg_event(conn, event),
            _ => {}
        }
    }

    fn handle_reg_event(&mut self, conn: &Connection, event: Event) {
        let ev = self.wl_registry.parse_event(event);
        match ev {
            WlRegistryEvent::Global {
                name,
                interface,
                version,
            } => match interface {
                "wl_output" => {
                    self.wl_outputs
                        .push(self.wl_registry.bind(&conn, name, interface, version));
                }
                "wl_compositor" => {
                    self.wl_compositor = self.wl_registry.bind(&conn, name, interface, version);
                }
                _ => {}
            },
            WlRegistryEvent::GlobalRemove { name } => {
                println!("Removed: {:?}", name);
            }
        }
        println!("{:?}", ev);
    }
}
