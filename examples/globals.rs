// #![allow(unused)]
use scratchway::{Connection, State, Object};
use scratchway::protocols::wayland::*;

fn main()  -> std::io::Result<()> {
    let conn = Connection::connect()?;
    let display = conn.display();
    let registry = display.get_registry(&conn);

    let mut app = App {
        registry,
    };

    conn.roundtrip(&mut app)?;

    // loop {
    //     conn.dispatch_events(&mut app)?;
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }
    Ok(())
}

struct App {
    registry: wl_registry::WlRegistry,
}

impl State for App {
    fn handle_event(&mut self, conn: &Connection, event: scratchway::events::WlEvent<'_>) {
        let display = conn.display();
        match event.header.id {
            1 => {
                println!("{:?}", display.parse_event(event, conn));
            }
            2 => {
                let ev = self.registry.parse_event(event, conn);
                println!("{:?}", ev);
            }
            _ => {}
        }
    }
}
