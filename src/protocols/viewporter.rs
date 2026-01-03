pub mod wp_viewporter {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WpViewporter {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WpViewporter {
        const INTERFACE: &'static str = "wp_viewporter";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_viewporter.destroy()", );
          }
        }
        pub fn get_viewport(&self, conn: &Connection, surface: &wl_surface::WlSurface, ) -> wp_viewport::WpViewport {
          let mut msg = Message::<16>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let surface_id = surface.id();
          msg.write_u32(surface_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_viewporter.get_viewport({}, {}, )", new_id,surface_id,);
          }
          Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WpViewporter {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WpViewporter {
            Self { id, interface: Self::INTERFACE }
        }
        fn id(&self) -> u32 {
            self.id
        }
        fn interface(&self) -> &'static str {
            self.interface
        }
        fn parse_event<'a>(&self, event: WlEvent<'a>, conn: &Connection) -> Self::Event<'a> {
            let parser = event.parser();
            match event.header.opcode {
                _ => unreachable!()
            }
        }
    }
}
pub mod wp_viewport {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WpViewport {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WpViewport {
        const INTERFACE: &'static str = "wp_viewport";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_viewport.destroy()", );
          }
        }
        pub fn set_source(&self, conn: &Connection, x: f32, y: f32, width: f32, height: f32, ) {
          let mut msg = Message::<24>::new(self.id, 1);
          msg.write_fixed(x);
          msg.write_fixed(y);
          msg.write_fixed(width);
          msg.write_fixed(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_viewport.set_source({:.2}, {:.2}, {:.2}, {:.2}, )", x,y,width,height,);
          }
        }
        pub fn set_destination(&self, conn: &Connection, width: i32, height: i32, ) {
          let mut msg = Message::<16>::new(self.id, 2);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_viewport.set_destination({}, {}, )", width,height,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WpViewport {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WpViewport {
            Self { id, interface: Self::INTERFACE }
        }
        fn id(&self) -> u32 {
            self.id
        }
        fn interface(&self) -> &'static str {
            self.interface
        }
        fn parse_event<'a>(&self, event: WlEvent<'a>, conn: &Connection) -> Self::Event<'a> {
            let parser = event.parser();
            match event.header.opcode {
                _ => unreachable!()
            }
        }
    }
}

