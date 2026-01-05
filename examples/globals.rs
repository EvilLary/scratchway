// #![allow(unused)]

use scratchway::prelude::*;
use scratchway::wayland::*;

fn main() -> std::io::Result<()> {
    let conn = Connection::connect()?;
    let display = conn.display();
    let registry = display.get_registry(conn.writer());

    let mut app = App { registry };

    conn.roundtrip(&mut app)
}

struct App {
    registry: wl_registry::WlRegistry,
}

impl App {
    fn on_wlregistry(&mut self, conn: &Connection, event: scratchway::events::WlEvent<'_>) {
        match self.registry.parse_event(conn.reader(), event) {
            wl_registry::Event::Global {
                interface, version, ..
            } => match interface {
                _ => {
                    println!("Global ==> {}, version: {}", interface, version);
                }
            },
            wl_registry::Event::GlobalRemove { .. } => {}
        }
    }
}

impl State for App {
    fn handle_event(&mut self, conn: &Connection, event: scratchway::events::WlEvent<'_>) {
        let display = conn.display();
        match event.header.id {
            1 => {
                println!("{:?}", display.parse_event(conn.reader(), event));
            }
            2 => {
                self.on_wlregistry(conn, event);
            }
            _ => {}
        }
    }
}
