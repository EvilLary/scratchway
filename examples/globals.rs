// #![allow(unused)]
use scratchway::connection::{Connection, State};
use scratchway::protocols::core::*;

fn main()  -> std::io::Result<()> {
    let mut conn = Connection::connect()?;
    let display = conn.display();
    let registry = display.get_registry(&conn);

    let mut app = App {
        registry,
    };

    conn.roundtrip(&mut app)?;

    loop {
        conn.dispatch_events(&mut app)?;
    }
    Ok(())
}

struct App {
    registry: WlRegistry,
}

impl State for App {
    fn handle_event(&mut self, _conn: &Connection, event: scratchway::events::Event<'_>) {
        let _ = self.registry.parse_event(event);
    }
}
