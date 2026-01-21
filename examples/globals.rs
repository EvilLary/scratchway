#![allow(unused)]

use scratchway::prelude::*;
use scratchway::wayland::*;

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;
    let display = conn.display();
    let registry = display.get_registry(&mut conn);

    registry.set_callback(
        &mut conn,
        |state: &mut (), conn: &mut Connection<()>, event: WlEvent| {
            let registry = wl_registry::WlRegistry::from_id(event.header.id);
            let event = registry.parse_event(conn, &event);
            match event {
                wl_registry::Event::Global {
                    interface,
                    version,
                    name,
                } => println!("Global ==> name: {:02}, {}", name, interface),
                wl_registry::Event::GlobalRemove { name } => {
                    println!("GlobalRemove ==> name: {}", name);
                },
            }
        },
    );

    display.set_callback(
        &mut conn,
        |state: &mut (), conn: &mut Connection<()>, event: WlEvent| {
            let event = conn.display().parse_event(conn, &event);
            match event {
                wl_display::Event::Error {
                    object_id,
                    code,
                    message,
                } => {},
                wl_display::Event::DeleteId { id } => {},
            }
        },
    );

    conn.roundtrip(&mut ())?;

    loop {
        conn.blocking_dispatch(&mut ())?;
    }
    Ok(())
}
