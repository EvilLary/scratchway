#![allow(unused)]

use scratchway::prelude::*;
use scratchway::wayland::*;

fn main() -> std::io::Result<()> {
    let mut conn = Connection::connect()?;
    let display = conn.display();
    let registry = display.get_registry(&conn);

    let mut app = App { registry };

    conn.add_callback(&app.registry, App::on_wlregistry);
    conn.add_callback(&display, App::on_wldisplay);


    conn.roundtrip(&mut app)?;

    loop {
        conn.dispatch_events(&mut app)?;
    }
}

struct App {
    registry: wl_registry::WlRegistry,
}

impl App {
    fn on_wlregistry(&mut self, conn: &mut Connection<Self>, event: scratchway::events::WlEvent) {
        match self.registry.parse_event(conn, &event) {
            wl_registry::Event::Global {
                interface,
                version,
                name,
            } => match interface {
                _ => {
                    println!("Global ==> name: {:02}, {}", name, interface);
                }
            },
            wl_registry::Event::GlobalRemove { name } => {
                println!("GlobalRemove ==> name: {}", name);
            }
        }
    }
    fn on_wldisplay(&mut self, conn: &mut Connection<Self>, event: scratchway::events::WlEvent) {
        match conn.display().parse_event(conn, &event) {
            wl_display::Event::Error { object_id, code, message } => {},
            wl_display::Event::DeleteId { id } => {},
        }
    }
}
