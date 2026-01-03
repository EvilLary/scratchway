use crate::protocols::xdg_shell::*;
pub mod zwlr_layer_shell_v1 {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct ZwlrLayerShellV1 {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl ZwlrLayerShellV1 {
        const INTERFACE: &'static str = "zwlr_layer_shell_v1";
        pub fn get_layer_surface(&self, conn: &Connection, surface: &wl_surface::WlSurface, output: Option<&wl_output::WlOutput>, layer: u32, namespace: &str, ) -> zwlr_layer_surface_v1::ZwlrLayerSurfaceV1 {
          let mut msg = Message::<60>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let surface_id = surface.id();
          msg.write_u32(surface_id);
          let output_id = output.map_or(0, |o| o.id());
          msg.write_u32(output_id);
          msg.write_u32(layer);
          msg.write_str(namespace);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_shell_v1.get_layer_surface({}, {}, {}, {}, {}, )", new_id,surface_id,output_id,layer,namespace,);
          }
          Object::from_id(new_id)
        }
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 1);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_shell_v1.destroy()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for ZwlrLayerShellV1 {
        type Event<'a> = Event;
        fn from_id(id: u32) -> ZwlrLayerShellV1 {
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
pub mod zwlr_layer_surface_v1 {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct ZwlrLayerSurfaceV1 {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl ZwlrLayerSurfaceV1 {
        const INTERFACE: &'static str = "zwlr_layer_surface_v1";
        pub fn set_size(&self, conn: &Connection, width: u32, height: u32, ) {
          let mut msg = Message::<16>::new(self.id, 0);
          msg.write_u32(width);
          msg.write_u32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_size({}, {}, )", width,height,);
          }
        }
        pub fn set_anchor(&self, conn: &Connection, anchor: u32, ) {
          let mut msg = Message::<12>::new(self.id, 1);
          msg.write_u32(anchor);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_anchor({}, )", anchor,);
          }
        }
        pub fn set_exclusive_zone(&self, conn: &Connection, zone: i32, ) {
          let mut msg = Message::<12>::new(self.id, 2);
          msg.write_i32(zone);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_exclusive_zone({}, )", zone,);
          }
        }
        pub fn set_margin(&self, conn: &Connection, top: i32, right: i32, bottom: i32, left: i32, ) {
          let mut msg = Message::<24>::new(self.id, 3);
          msg.write_i32(top);
          msg.write_i32(right);
          msg.write_i32(bottom);
          msg.write_i32(left);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_margin({}, {}, {}, {}, )", top,right,bottom,left,);
          }
        }
        pub fn set_keyboard_interactivity(&self, conn: &Connection, keyboard_interactivity: u32, ) {
          let mut msg = Message::<12>::new(self.id, 4);
          msg.write_u32(keyboard_interactivity);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_keyboard_interactivity({}, )", keyboard_interactivity,);
          }
        }
        pub fn get_popup(&self, conn: &Connection, popup: &xdg_popup::XdgPopup, ) {
          let mut msg = Message::<12>::new(self.id, 5);
          let popup_id = popup.id();
          msg.write_u32(popup_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.get_popup({}, )", popup_id,);
          }
        }
        pub fn ack_configure(&self, conn: &Connection, serial: u32, ) {
          let mut msg = Message::<12>::new(self.id, 6);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.ack_configure({}, )", serial,);
          }
        }
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 7);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.destroy()", );
          }
        }
        pub fn set_layer(&self, conn: &Connection, layer: u32, ) {
          let mut msg = Message::<12>::new(self.id, 8);
          msg.write_u32(layer);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_layer({}, )", layer,);
          }
        }
        pub fn set_exclusive_edge(&self, conn: &Connection, edge: u32, ) {
          let mut msg = Message::<12>::new(self.id, 9);
          msg.write_u32(edge);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "zwlr_layer_surface_v1.set_exclusive_edge({}, )", edge,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Configure {
            serial: u32,
            width: u32,
            height: u32,
        },
        Closed,
    }
    impl Object for ZwlrLayerSurfaceV1 {
        type Event<'a> = Event;
        fn from_id(id: u32) -> ZwlrLayerSurfaceV1 {
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
                 0 => {
                     let serial = parser.get_u32();
                     let width = parser.get_u32();
                     let height = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> zwlr_layer_surface_v1.configure({}, {}, {}, )", serial,width,height,);
                      }
                     Self::Event::Configure { serial, width, height,  }
                 }
                 1 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> zwlr_layer_surface_v1.closed()", );
                      }
                     Self::Event::Closed
                 }
                _ => unreachable!()
            }
        }
    }
}

