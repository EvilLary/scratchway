#![allow(unused)]

use std::io::Read;

use scratchway::connection::Connection;
use scratchway::core::*;
use scratchway::events::{Event, EventDataParser, EventIter};

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&mut conn);
    let mut state = State {
        wl_display,
        wl_registry,
        wl_outputs: Vec::new(),
    };
    conn.flush()?;

    // wl_display.sync(&mut conn);
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
}

impl State {
    fn handle_event(&mut self, conn: &Connection, event: Event) {
        match event.header.id {
            id if id == self.wl_display.id => {
                let ev = self.wl_display.parse_event(event);
                println!("{:?}", ev);
            }
            id if id == self.wl_registry.id => self.handle_reg_event(conn, event),
            id if id == self.wl_outputs.first().unwrap().id => {
                println!("{:?}", self.wl_outputs.first().unwrap().parse_event(event));
            }
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
                    let wl_compositor: WlCompositor =
                        self.wl_registry.bind(&conn, name, interface, version);
                }
                _ => {}
            },
            WlRegistryEvent::GlobalRemove { name } => {}
        }
        println!("{:?}", ev);
    }
}
