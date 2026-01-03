pub mod wp_single_pixel_buffer_manager_v1 {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WpSinglePixelBufferManagerV1 {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WpSinglePixelBufferManagerV1 {
        const INTERFACE: &'static str = "wp_single_pixel_buffer_manager_v1";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_single_pixel_buffer_manager_v1.destroy()", );
          }
        }
        pub fn create_u32_rgba_buffer(&self, conn: &Connection, r: u32, g: u32, b: u32, a: u32, ) -> wl_buffer::WlBuffer {
          let mut msg = Message::<28>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.write_u32(r);
          msg.write_u32(g);
          msg.write_u32(b);
          msg.write_u32(a);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wp_single_pixel_buffer_manager_v1.create_u32_rgba_buffer({}, {}, {}, {}, {}, )", new_id,r,g,b,a,);
          }
          Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WpSinglePixelBufferManagerV1 {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WpSinglePixelBufferManagerV1 {
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

