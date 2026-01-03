pub mod xdg_wm_base {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct XdgWmBase {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl XdgWmBase {
        const INTERFACE: &'static str = "xdg_wm_base";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_wm_base.destroy()", );
          }
        }
        pub fn create_positioner(&self, conn: &Connection, ) -> xdg_positioner::XdgPositioner {
          let mut msg = Message::<12>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_wm_base.create_positioner({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn get_xdg_surface(&self, conn: &Connection, surface: &wl_surface::WlSurface, ) -> xdg_surface::XdgSurface {
          let mut msg = Message::<16>::new(self.id, 2);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let surface_id = surface.id();
          msg.write_u32(surface_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_wm_base.get_xdg_surface({}, {}, )", new_id,surface_id,);
          }
          Object::from_id(new_id)
        }
        pub fn pong(&self, conn: &Connection, serial: u32, ) {
          let mut msg = Message::<12>::new(self.id, 3);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_wm_base.pong({}, )", serial,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Ping {
            serial: u32,
        },
    }
    impl Object for XdgWmBase {
        type Event<'a> = Event;
        fn from_id(id: u32) -> XdgWmBase {
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
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_wm_base.ping({}, )", serial,);
                      }
                     Self::Event::Ping { serial,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod xdg_positioner {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct XdgPositioner {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl XdgPositioner {
        const INTERFACE: &'static str = "xdg_positioner";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.destroy()", );
          }
        }
        pub fn set_size(&self, conn: &Connection, width: i32, height: i32, ) {
          let mut msg = Message::<16>::new(self.id, 1);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_size({}, {}, )", width,height,);
          }
        }
        pub fn set_anchor_rect(&self, conn: &Connection, x: i32, y: i32, width: i32, height: i32, ) {
          let mut msg = Message::<24>::new(self.id, 2);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_anchor_rect({}, {}, {}, {}, )", x,y,width,height,);
          }
        }
        pub fn set_anchor(&self, conn: &Connection, anchor: u32, ) {
          let mut msg = Message::<12>::new(self.id, 3);
          msg.write_u32(anchor);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_anchor({}, )", anchor,);
          }
        }
        pub fn set_gravity(&self, conn: &Connection, gravity: u32, ) {
          let mut msg = Message::<12>::new(self.id, 4);
          msg.write_u32(gravity);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_gravity({}, )", gravity,);
          }
        }
        pub fn set_constraint_adjustment(&self, conn: &Connection, constraint_adjustment: u32, ) {
          let mut msg = Message::<12>::new(self.id, 5);
          msg.write_u32(constraint_adjustment);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_constraint_adjustment({}, )", constraint_adjustment,);
          }
        }
        pub fn set_offset(&self, conn: &Connection, x: i32, y: i32, ) {
          let mut msg = Message::<16>::new(self.id, 6);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_offset({}, {}, )", x,y,);
          }
        }
        pub fn set_reactive(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 7);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_reactive()", );
          }
        }
        pub fn set_parent_size(&self, conn: &Connection, parent_width: i32, parent_height: i32, ) {
          let mut msg = Message::<16>::new(self.id, 8);
          msg.write_i32(parent_width);
          msg.write_i32(parent_height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_parent_size({}, {}, )", parent_width,parent_height,);
          }
        }
        pub fn set_parent_configure(&self, conn: &Connection, serial: u32, ) {
          let mut msg = Message::<12>::new(self.id, 9);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_positioner.set_parent_configure({}, )", serial,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for XdgPositioner {
        type Event<'a> = Event;
        fn from_id(id: u32) -> XdgPositioner {
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
pub mod xdg_surface {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct XdgSurface {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl XdgSurface {
        const INTERFACE: &'static str = "xdg_surface";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_surface.destroy()", );
          }
        }
        pub fn get_toplevel(&self, conn: &Connection, ) -> xdg_toplevel::XdgToplevel {
          let mut msg = Message::<12>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_surface.get_toplevel({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn get_popup(&self, conn: &Connection, parent: Option<&xdg_surface::XdgSurface>, positioner: &xdg_positioner::XdgPositioner, ) -> xdg_popup::XdgPopup {
          let mut msg = Message::<20>::new(self.id, 2);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let parent_id = parent.map_or(0, |o| o.id());
          msg.write_u32(parent_id);
          let positioner_id = positioner.id();
          msg.write_u32(positioner_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_surface.get_popup({}, {}, {}, )", new_id,parent_id,positioner_id,);
          }
          Object::from_id(new_id)
        }
        pub fn set_window_geometry(&self, conn: &Connection, x: i32, y: i32, width: i32, height: i32, ) {
          let mut msg = Message::<24>::new(self.id, 3);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_surface.set_window_geometry({}, {}, {}, {}, )", x,y,width,height,);
          }
        }
        pub fn ack_configure(&self, conn: &Connection, serial: u32, ) {
          let mut msg = Message::<12>::new(self.id, 4);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_surface.ack_configure({}, )", serial,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Configure {
            serial: u32,
        },
    }
    impl Object for XdgSurface {
        type Event<'a> = Event;
        fn from_id(id: u32) -> XdgSurface {
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
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_surface.configure({}, )", serial,);
                      }
                     Self::Event::Configure { serial,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod xdg_toplevel {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct XdgToplevel {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl XdgToplevel {
        const INTERFACE: &'static str = "xdg_toplevel";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.destroy()", );
          }
        }
        pub fn set_parent(&self, conn: &Connection, parent: Option<&xdg_toplevel::XdgToplevel>, ) {
          let mut msg = Message::<12>::new(self.id, 1);
          let parent_id = parent.map_or(0, |o| o.id());
          msg.write_u32(parent_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_parent({}, )", parent_id,);
          }
        }
        pub fn set_title(&self, conn: &Connection, title: &str, ) {
          let mut msg = Message::<44>::new(self.id, 2);
          msg.write_str(title);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_title({}, )", title,);
          }
        }
        pub fn set_app_id(&self, conn: &Connection, app_id: &str, ) {
          let mut msg = Message::<44>::new(self.id, 3);
          msg.write_str(app_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_app_id({}, )", app_id,);
          }
        }
        pub fn show_window_menu(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, x: i32, y: i32, ) {
          let mut msg = Message::<24>::new(self.id, 4);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.show_window_menu({}, {}, {}, {}, )", seat_id,serial,x,y,);
          }
        }
        pub fn move_(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, ) {
          let mut msg = Message::<16>::new(self.id, 5);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.move({}, {}, )", seat_id,serial,);
          }
        }
        pub fn resize(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, edges: u32, ) {
          let mut msg = Message::<20>::new(self.id, 6);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          msg.write_u32(edges);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.resize({}, {}, {}, )", seat_id,serial,edges,);
          }
        }
        pub fn set_max_size(&self, conn: &Connection, width: i32, height: i32, ) {
          let mut msg = Message::<16>::new(self.id, 7);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_max_size({}, {}, )", width,height,);
          }
        }
        pub fn set_min_size(&self, conn: &Connection, width: i32, height: i32, ) {
          let mut msg = Message::<16>::new(self.id, 8);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_min_size({}, {}, )", width,height,);
          }
        }
        pub fn set_maximized(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 9);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_maximized()", );
          }
        }
        pub fn unset_maximized(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 10);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.unset_maximized()", );
          }
        }
        pub fn set_fullscreen(&self, conn: &Connection, output: Option<&wl_output::WlOutput>, ) {
          let mut msg = Message::<12>::new(self.id, 11);
          let output_id = output.map_or(0, |o| o.id());
          msg.write_u32(output_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_fullscreen({}, )", output_id,);
          }
        }
        pub fn unset_fullscreen(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 12);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.unset_fullscreen()", );
          }
        }
        pub fn set_minimized(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 13);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_toplevel.set_minimized()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Configure {
            width: i32,
            height: i32,
            states: &'a [u32],
        },
        Close,
        ConfigureBounds {
            width: i32,
            height: i32,
        },
        WmCapabilities {
            capabilities: &'a [u32],
        },
    }
    impl Object for XdgToplevel {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> XdgToplevel {
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
                     let width = parser.get_i32();
                     let height = parser.get_i32();
                     let states = parser.get_array_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_toplevel.configure({}, {}, {:?}, )", width,height,states,);
                      }
                     Self::Event::Configure { width, height, states,  }
                 }
                 1 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_toplevel.close()", );
                      }
                     Self::Event::Close
                 }
                 2 => {
                     let width = parser.get_i32();
                     let height = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_toplevel.configure_bounds({}, {}, )", width,height,);
                      }
                     Self::Event::ConfigureBounds { width, height,  }
                 }
                 3 => {
                     let capabilities = parser.get_array_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_toplevel.wm_capabilities({:?}, )", capabilities,);
                      }
                     Self::Event::WmCapabilities { capabilities,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod xdg_popup {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct XdgPopup {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl XdgPopup {
        const INTERFACE: &'static str = "xdg_popup";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_popup.destroy()", );
          }
        }
        pub fn grab(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, ) {
          let mut msg = Message::<16>::new(self.id, 1);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_popup.grab({}, {}, )", seat_id,serial,);
          }
        }
        pub fn reposition(&self, conn: &Connection, positioner: &xdg_positioner::XdgPositioner, token: u32, ) {
          let mut msg = Message::<16>::new(self.id, 2);
          let positioner_id = positioner.id();
          msg.write_u32(positioner_id);
          msg.write_u32(token);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "xdg_popup.reposition({}, {}, )", positioner_id,token,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Configure {
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        PopupDone,
        Repositioned {
            token: u32,
        },
    }
    impl Object for XdgPopup {
        type Event<'a> = Event;
        fn from_id(id: u32) -> XdgPopup {
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
                     let x = parser.get_i32();
                     let y = parser.get_i32();
                     let width = parser.get_i32();
                     let height = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_popup.configure({}, {}, {}, {}, )", x,y,width,height,);
                      }
                     Self::Event::Configure { x, y, width, height,  }
                 }
                 1 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_popup.popup_done()", );
                      }
                     Self::Event::PopupDone
                 }
                 2 => {
                     let token = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> xdg_popup.repositioned({}, )", token,);
                      }
                     Self::Event::Repositioned { token,  }
                 }
                _ => unreachable!()
            }
        }
    }
}

