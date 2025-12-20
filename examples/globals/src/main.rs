#![allow(unused)]

use std::io::Read;

use scratchway::connection::Connection;
use scratchway::core::{wl_display::*, wl_registry::*};
use scratchway::events::{Event, EventDataParser, EventIter};

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;

    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(&mut conn);
    conn.flush()?;

    let mut events = conn.blocking_read();
    while let Some(event) = events.next() {
        match event.header.id {
            id if id == wl_display.id => {
                let ev = wl_display.parse_event(event);
                println!("{:?}", ev);
            }
            id if id == wl_registry.id => {
                let ev = wl_registry.parse_event(event);
                println!("{:?}", ev);
            }
            _ => {}
        }
    }
    // println!("Hello, world!");
    Ok(())
}

struct State {
    wl_display: WlDisplay,
    wl_registry: WlRegistry,
}
