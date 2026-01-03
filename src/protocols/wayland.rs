#![allow(unused)]

pub mod wl_display {
    use super::*;
    use crate::connection::Connection;
    use crate::connection::Object;
    use crate::events::*;

    #[derive(Debug)]
    pub struct WlDisplay {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl Default for WlDisplay {
        fn default() -> Self {
            Object::from_id(1)
        }
    }
    impl WlDisplay {
        const INTERFACE: &'static str = "wl_display";
        pub fn sync(&self, conn: &Connection) -> wl_callback::WlCallback {
            let mut msg = Message::<12>::new(self.id, 0);
            let new_id = conn.new_id();
            msg.write_u32(new_id);
            msg.build();
            conn.write_request(msg);
            if *crate::connection::DEBUG {
                crate::log!(WAYLAND, "==> wl_display.sync({})", new_id);
            }
            Object::from_id(new_id)
        }
        pub fn get_registry(&self, conn: &Connection) -> wl_registry::WlRegistry {
            let mut msg = Message::<12>::new(self.id, 1);
            let new_id = conn.new_id();
            msg.write_u32(new_id);
            msg.build();
            conn.write_request(msg);
            if *crate::connection::DEBUG {
                crate::log!(WAYLAND, "==> wl_display.get_registry({})", new_id);
            }
            Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Error {
            object_id: u32,
            code: u32,
            message: &'a str,
        },
        DeleteId {
            id: u32,
        },
    }
    impl Object for WlDisplay {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> Self {
            Self {
                id,
                interface: Self::INTERFACE,
            }
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
                    let object_id = parser.get_u32();
                    let code = parser.get_u32();
                    let message = parser.get_string();
                    if *crate::connection::DEBUG {
                        crate::log!(
                            WAYLAND,
                            "==> wl_display.error({}, {}, {})",
                            object_id,
                            code,
                            message
                        );
                    }
                    Self::Event::Error {
                        object_id,
                        code,
                        message,
                    }
                }
                1 => {
                    let id = parser.get_u32();
                    if *crate::connection::DEBUG {
                        crate::log!(WAYLAND, "==> wl_display.delete_id({})", id);
                    }
                    Self::Event::DeleteId { id }
                }
                _ => unreachable!(),
            }
        }
    }
}

pub mod wl_registry {
    use super::*;
    use crate::connection::Connection;
    use crate::connection::Object;
    use crate::events::*;

    #[derive(Debug)]
    pub struct WlRegistry {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlRegistry {
        const INTERFACE: &'static str = "wl_registry";
        pub fn bind<O: Object>(
            &self, conn: &Connection, name: u32, interface: &str, version: u32,
        ) -> O {
            let mut msg = Message::<64>::new(self.id, 0);
            let new_id = conn.new_id();
            msg.write_u32(name);
            msg.write_str(interface);
            msg.write_u32(version);
            msg.write_u32(new_id);
            msg.build();
            conn.write_request(msg);
            if *crate::connection::DEBUG {
                crate::log!(
                    WAYLAND,
                    "wl_registry.bind({}, {}, {}, {})",
                    new_id,
                    name,
                    interface,
                    version
                );
            }
            Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Global {
            name: u32,
            interface: &'a str,
            version: u32,
        },
        GlobalRemove {
            name: u32,
        },
    }
    impl Object for WlRegistry {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> Self {
            Self {
                id,
                interface: Self::INTERFACE,
            }
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
                    let name = parser.get_u32();
                    let interface = parser.get_string();
                    let version = parser.get_u32();
                    if *crate::connection::DEBUG {
                        crate::log!(
                            WAYLAND,
                            "==> wl_registry.global({}, {}, {})",
                            name,
                            interface,
                            version
                        );
                    }
                    Self::Event::Global {
                        name,
                        interface,
                        version,
                    }
                }
                1 => {
                    let name = parser.get_u32();
                    if *crate::connection::DEBUG {
                        crate::log!(WAYLAND, "==> wl_registry.global_remove({})", name);
                    }
                    Self::Event::GlobalRemove { name }
                }
                _ => unreachable!(),
            }
        }
    }
}


pub mod wl_callback {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlCallback {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlCallback {
        const INTERFACE: &'static str = "wl_callback";
    }
    #[derive(Debug)]
    pub enum Event {
        Done {
            callback_data: u32,
        },
    }
    impl Object for WlCallback {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlCallback {
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
                     let callback_data = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_callback.done({}, )", callback_data,);
                      }
                     Self::Event::Done { callback_data,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_compositor {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlCompositor {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlCompositor {
        const INTERFACE: &'static str = "wl_compositor";
        pub fn create_surface(&self, conn: &Connection, ) -> wl_surface::WlSurface {
          let mut msg = Message::<12>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_compositor.create_surface({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn create_region(&self, conn: &Connection, ) -> wl_region::WlRegion {
          let mut msg = Message::<12>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_compositor.create_region({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlCompositor {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlCompositor {
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
pub mod wl_shm_pool {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlShmPool {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlShmPool {
        const INTERFACE: &'static str = "wl_shm_pool";
        pub fn create_buffer(&self, conn: &Connection, offset: i32, width: i32, height: i32, stride: i32, format: u32, ) -> wl_buffer::WlBuffer {
          let mut msg = Message::<32>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.write_i32(offset);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.write_i32(stride);
          msg.write_u32(format);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shm_pool.create_buffer({}, {}, {}, {}, {}, {}, )", new_id,offset,width,height,stride,format,);
          }
          Object::from_id(new_id)
        }
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 1);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shm_pool.destroy()", );
          }
        }
        pub fn resize(&self, conn: &Connection, size: i32, ) {
          let mut msg = Message::<12>::new(self.id, 2);
          msg.write_i32(size);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shm_pool.resize({}, )", size,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlShmPool {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlShmPool {
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
pub mod wl_shm {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlShm {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlShm {
        const INTERFACE: &'static str = "wl_shm";
        pub fn create_pool(&self, conn: &Connection, fd: i32, size: i32, ) -> wl_shm_pool::WlShmPool {
          let mut msg = Message::<16>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          conn.add_fd(fd);
          msg.write_i32(size);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shm.create_pool({}, {}, {}, )", new_id,fd,size,);
          }
          Object::from_id(new_id)
        }
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 1);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shm.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Format {
            format: u32,
        },
    }
    impl Object for WlShm {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlShm {
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
                     let format = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_shm.format({}, )", format,);
                      }
                     Self::Event::Format { format,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_buffer {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlBuffer {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlBuffer {
        const INTERFACE: &'static str = "wl_buffer";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_buffer.destroy()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Release,
    }
    impl Object for WlBuffer {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlBuffer {
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
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_buffer.release()", );
                      }
                     Self::Event::Release
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_data_offer {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlDataOffer {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlDataOffer {
        const INTERFACE: &'static str = "wl_data_offer";
        pub fn accept(&self, conn: &Connection, serial: u32, mime_type: &str, ) {
          let mut msg = Message::<48>::new(self.id, 0);
          msg.write_u32(serial);
          todo!();
          msg.write_str(mime_type);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_offer.accept({}, {}, )", serial,mime_type,);
          }
        }
        pub fn receive(&self, conn: &Connection, mime_type: &str, fd: i32, ) {
          let mut msg = Message::<44>::new(self.id, 1);
          msg.write_str(mime_type);
          conn.add_fd(fd);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_offer.receive({}, {}, )", mime_type,fd,);
          }
        }
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 2);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_offer.destroy()", );
          }
        }
        pub fn finish(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 3);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_offer.finish()", );
          }
        }
        pub fn set_actions(&self, conn: &Connection, dnd_actions: u32, preferred_action: u32, ) {
          let mut msg = Message::<16>::new(self.id, 4);
          msg.write_u32(dnd_actions);
          msg.write_u32(preferred_action);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_offer.set_actions({}, {}, )", dnd_actions,preferred_action,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Offer {
            mime_type: &'a str,
        },
        SourceActions {
            source_actions: u32,
        },
        Action {
            dnd_action: u32,
        },
    }
    impl Object for WlDataOffer {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> WlDataOffer {
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
                     let mime_type = parser.get_string();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_offer.offer({}, )", mime_type,);
                      }
                     Self::Event::Offer { mime_type,  }
                 }
                 1 => {
                     let source_actions = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_offer.source_actions({}, )", source_actions,);
                      }
                     Self::Event::SourceActions { source_actions,  }
                 }
                 2 => {
                     let dnd_action = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_offer.action({}, )", dnd_action,);
                      }
                     Self::Event::Action { dnd_action,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_data_source {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlDataSource {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlDataSource {
        const INTERFACE: &'static str = "wl_data_source";
        pub fn offer(&self, conn: &Connection, mime_type: &str, ) {
          let mut msg = Message::<44>::new(self.id, 0);
          msg.write_str(mime_type);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_source.offer({}, )", mime_type,);
          }
        }
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 1);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_source.destroy()", );
          }
        }
        pub fn set_actions(&self, conn: &Connection, dnd_actions: u32, ) {
          let mut msg = Message::<12>::new(self.id, 2);
          msg.write_u32(dnd_actions);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_source.set_actions({}, )", dnd_actions,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Target {
            mime_type: &'a str,
        },
        Send {
            mime_type: &'a str,
            fd: std::os::fd::OwnedFd, 
        },
        Cancelled,
        DndDropPerformed,
        DndFinished,
        Action {
            dnd_action: u32,
        },
    }
    impl Object for WlDataSource {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> WlDataSource {
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
                     let mime_type = parser.get_string();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_source.target({}, )", mime_type,);
                      }
                     Self::Event::Target { mime_type,  }
                 }
                 1 => {
                     let mime_type = parser.get_string();
                     let fd = conn.get_fd().unwrap();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_source.send({}, {:?}, )", mime_type,fd,);
                      }
                     Self::Event::Send { mime_type, fd,  }
                 }
                 2 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_source.cancelled()", );
                      }
                     Self::Event::Cancelled
                 }
                 3 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_source.dnd_drop_performed()", );
                      }
                     Self::Event::DndDropPerformed
                 }
                 4 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_source.dnd_finished()", );
                      }
                     Self::Event::DndFinished
                 }
                 5 => {
                     let dnd_action = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_source.action({}, )", dnd_action,);
                      }
                     Self::Event::Action { dnd_action,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_data_device {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlDataDevice {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlDataDevice {
        const INTERFACE: &'static str = "wl_data_device";
        pub fn start_drag(&self, conn: &Connection, source: Option<&wl_data_source::WlDataSource>, origin: &wl_surface::WlSurface, icon: Option<&wl_surface::WlSurface>, serial: u32, ) {
          let mut msg = Message::<24>::new(self.id, 0);
          let source_id = source.map_or(0, |o| o.id());
          msg.write_u32(source_id);
          let origin_id = origin.id();
          msg.write_u32(origin_id);
          let icon_id = icon.map_or(0, |o| o.id());
          msg.write_u32(icon_id);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_device.start_drag({}, {}, {}, {}, )", source_id,origin_id,icon_id,serial,);
          }
        }
        pub fn set_selection(&self, conn: &Connection, source: Option<&wl_data_source::WlDataSource>, serial: u32, ) {
          let mut msg = Message::<16>::new(self.id, 1);
          let source_id = source.map_or(0, |o| o.id());
          msg.write_u32(source_id);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_device.set_selection({}, {}, )", source_id,serial,);
          }
        }
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 2);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_device.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        DataOffer {
            id: u32, 
        },
        Enter {
            serial: u32,
            surface: u32, 
            x: f32,
            y: f32,
            id: u32, 
        },
        Leave,
        Motion {
            time: u32,
            x: f32,
            y: f32,
        },
        Drop,
        Selection {
            id: u32, 
        },
    }
    impl Object for WlDataDevice {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlDataDevice {
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
                     todo!();
                     let id = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_device.data_offer({}, )", id,);
                      }
                     Self::Event::DataOffer { id,  }
                 }
                 1 => {
                     let serial = parser.get_u32();
                     let surface = parser.get_u32();
                     let x = parser.get_fixed();
                     let y = parser.get_fixed();
                     let id = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_device.enter({}, {}, {:.2}, {:.2}, {}, )", serial,surface,x,y,id,);
                      }
                     Self::Event::Enter { serial, surface, x, y, id,  }
                 }
                 2 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_device.leave()", );
                      }
                     Self::Event::Leave
                 }
                 3 => {
                     let time = parser.get_u32();
                     let x = parser.get_fixed();
                     let y = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_device.motion({}, {:.2}, {:.2}, )", time,x,y,);
                      }
                     Self::Event::Motion { time, x, y,  }
                 }
                 4 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_device.drop()", );
                      }
                     Self::Event::Drop
                 }
                 5 => {
                     let id = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_data_device.selection({}, )", id,);
                      }
                     Self::Event::Selection { id,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_data_device_manager {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlDataDeviceManager {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlDataDeviceManager {
        const INTERFACE: &'static str = "wl_data_device_manager";
        pub fn create_data_source(&self, conn: &Connection, ) -> wl_data_source::WlDataSource {
          let mut msg = Message::<12>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_device_manager.create_data_source({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn get_data_device(&self, conn: &Connection, seat: &wl_seat::WlSeat, ) -> wl_data_device::WlDataDevice {
          let mut msg = Message::<16>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_data_device_manager.get_data_device({}, {}, )", new_id,seat_id,);
          }
          Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlDataDeviceManager {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlDataDeviceManager {
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
pub mod wl_shell {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlShell {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlShell {
        const INTERFACE: &'static str = "wl_shell";
        pub fn get_shell_surface(&self, conn: &Connection, surface: &wl_surface::WlSurface, ) -> wl_shell_surface::WlShellSurface {
          let mut msg = Message::<16>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let surface_id = surface.id();
          msg.write_u32(surface_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell.get_shell_surface({}, {}, )", new_id,surface_id,);
          }
          Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlShell {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlShell {
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
pub mod wl_shell_surface {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlShellSurface {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlShellSurface {
        const INTERFACE: &'static str = "wl_shell_surface";
        pub fn pong(&self, conn: &Connection, serial: u32, ) {
          let mut msg = Message::<12>::new(self.id, 0);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.pong({}, )", serial,);
          }
        }
        pub fn move_(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, ) {
          let mut msg = Message::<16>::new(self.id, 1);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.move({}, {}, )", seat_id,serial,);
          }
        }
        pub fn resize(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, edges: u32, ) {
          let mut msg = Message::<20>::new(self.id, 2);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          msg.write_u32(edges);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.resize({}, {}, {}, )", seat_id,serial,edges,);
          }
        }
        pub fn set_toplevel(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 3);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_toplevel()", );
          }
        }
        pub fn set_transient(&self, conn: &Connection, parent: &wl_surface::WlSurface, x: i32, y: i32, flags: u32, ) {
          let mut msg = Message::<24>::new(self.id, 4);
          let parent_id = parent.id();
          msg.write_u32(parent_id);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_u32(flags);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_transient({}, {}, {}, {}, )", parent_id,x,y,flags,);
          }
        }
        pub fn set_fullscreen(&self, conn: &Connection, method: u32, framerate: u32, output: Option<&wl_output::WlOutput>, ) {
          let mut msg = Message::<20>::new(self.id, 5);
          msg.write_u32(method);
          msg.write_u32(framerate);
          let output_id = output.map_or(0, |o| o.id());
          msg.write_u32(output_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_fullscreen({}, {}, {}, )", method,framerate,output_id,);
          }
        }
        pub fn set_popup(&self, conn: &Connection, seat: &wl_seat::WlSeat, serial: u32, parent: &wl_surface::WlSurface, x: i32, y: i32, flags: u32, ) {
          let mut msg = Message::<32>::new(self.id, 6);
          let seat_id = seat.id();
          msg.write_u32(seat_id);
          msg.write_u32(serial);
          let parent_id = parent.id();
          msg.write_u32(parent_id);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_u32(flags);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_popup({}, {}, {}, {}, {}, {}, )", seat_id,serial,parent_id,x,y,flags,);
          }
        }
        pub fn set_maximized(&self, conn: &Connection, output: Option<&wl_output::WlOutput>, ) {
          let mut msg = Message::<12>::new(self.id, 7);
          let output_id = output.map_or(0, |o| o.id());
          msg.write_u32(output_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_maximized({}, )", output_id,);
          }
        }
        pub fn set_title(&self, conn: &Connection, title: &str, ) {
          let mut msg = Message::<44>::new(self.id, 8);
          msg.write_str(title);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_title({}, )", title,);
          }
        }
        pub fn set_class(&self, conn: &Connection, class_: &str, ) {
          let mut msg = Message::<44>::new(self.id, 9);
          msg.write_str(class_);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_shell_surface.set_class({}, )", class_,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Ping {
            serial: u32,
        },
        Configure {
            edges: u32,
            width: i32,
            height: i32,
        },
        PopupDone,
    }
    impl Object for WlShellSurface {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlShellSurface {
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
                          crate::log!(WAYLAND, "==> wl_shell_surface.ping({}, )", serial,);
                      }
                     Self::Event::Ping { serial,  }
                 }
                 1 => {
                     let edges = parser.get_u32();
                     let width = parser.get_i32();
                     let height = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_shell_surface.configure({}, {}, {}, )", edges,width,height,);
                      }
                     Self::Event::Configure { edges, width, height,  }
                 }
                 2 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_shell_surface.popup_done()", );
                      }
                     Self::Event::PopupDone
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_surface {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlSurface {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlSurface {
        const INTERFACE: &'static str = "wl_surface";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.destroy()", );
          }
        }
        pub fn attach(&self, conn: &Connection, buffer: Option<&wl_buffer::WlBuffer>, x: i32, y: i32, ) {
          let mut msg = Message::<20>::new(self.id, 1);
          let buffer_id = buffer.map_or(0, |o| o.id());
          msg.write_u32(buffer_id);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.attach({}, {}, {}, )", buffer_id,x,y,);
          }
        }
        pub fn damage(&self, conn: &Connection, x: i32, y: i32, width: i32, height: i32, ) {
          let mut msg = Message::<24>::new(self.id, 2);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.damage({}, {}, {}, {}, )", x,y,width,height,);
          }
        }
        pub fn frame(&self, conn: &Connection, ) -> wl_callback::WlCallback {
          let mut msg = Message::<12>::new(self.id, 3);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.frame({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn set_opaque_region(&self, conn: &Connection, region: Option<&wl_region::WlRegion>, ) {
          let mut msg = Message::<12>::new(self.id, 4);
          let region_id = region.map_or(0, |o| o.id());
          msg.write_u32(region_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.set_opaque_region({}, )", region_id,);
          }
        }
        pub fn set_input_region(&self, conn: &Connection, region: Option<&wl_region::WlRegion>, ) {
          let mut msg = Message::<12>::new(self.id, 5);
          let region_id = region.map_or(0, |o| o.id());
          msg.write_u32(region_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.set_input_region({}, )", region_id,);
          }
        }
        pub fn commit(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 6);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.commit()", );
          }
        }
        pub fn set_buffer_transform(&self, conn: &Connection, transform: u32, ) {
          let mut msg = Message::<12>::new(self.id, 7);
          msg.write_u32(transform);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.set_buffer_transform({}, )", transform,);
          }
        }
        pub fn set_buffer_scale(&self, conn: &Connection, scale: i32, ) {
          let mut msg = Message::<12>::new(self.id, 8);
          msg.write_i32(scale);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.set_buffer_scale({}, )", scale,);
          }
        }
        pub fn damage_buffer(&self, conn: &Connection, x: i32, y: i32, width: i32, height: i32, ) {
          let mut msg = Message::<24>::new(self.id, 9);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.damage_buffer({}, {}, {}, {}, )", x,y,width,height,);
          }
        }
        pub fn offset(&self, conn: &Connection, x: i32, y: i32, ) {
          let mut msg = Message::<16>::new(self.id, 10);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_surface.offset({}, {}, )", x,y,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Enter {
            output: u32, 
        },
        Leave {
            output: u32, 
        },
        PreferredBufferScale {
            factor: i32,
        },
        PreferredBufferTransform {
            transform: u32,
        },
    }
    impl Object for WlSurface {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlSurface {
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
                     let output = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_surface.enter({}, )", output,);
                      }
                     Self::Event::Enter { output,  }
                 }
                 1 => {
                     let output = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_surface.leave({}, )", output,);
                      }
                     Self::Event::Leave { output,  }
                 }
                 2 => {
                     let factor = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_surface.preferred_buffer_scale({}, )", factor,);
                      }
                     Self::Event::PreferredBufferScale { factor,  }
                 }
                 3 => {
                     let transform = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_surface.preferred_buffer_transform({}, )", transform,);
                      }
                     Self::Event::PreferredBufferTransform { transform,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_seat {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlSeat {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlSeat {
        const INTERFACE: &'static str = "wl_seat";
        pub fn get_pointer(&self, conn: &Connection, ) -> wl_pointer::WlPointer {
          let mut msg = Message::<12>::new(self.id, 0);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_seat.get_pointer({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn get_keyboard(&self, conn: &Connection, ) -> wl_keyboard::WlKeyboard {
          let mut msg = Message::<12>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_seat.get_keyboard({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn get_touch(&self, conn: &Connection, ) -> wl_touch::WlTouch {
          let mut msg = Message::<12>::new(self.id, 2);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_seat.get_touch({}, )", new_id,);
          }
          Object::from_id(new_id)
        }
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 3);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_seat.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Capabilities {
            capabilities: u32,
        },
        Name {
            name: &'a str,
        },
    }
    impl Object for WlSeat {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> WlSeat {
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
                     let capabilities = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_seat.capabilities({}, )", capabilities,);
                      }
                     Self::Event::Capabilities { capabilities,  }
                 }
                 1 => {
                     let name = parser.get_string();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_seat.name({}, )", name,);
                      }
                     Self::Event::Name { name,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_pointer {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlPointer {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlPointer {
        const INTERFACE: &'static str = "wl_pointer";
        pub fn set_cursor(&self, conn: &Connection, serial: u32, surface: Option<&wl_surface::WlSurface>, hotspot_x: i32, hotspot_y: i32, ) {
          let mut msg = Message::<24>::new(self.id, 0);
          msg.write_u32(serial);
          let surface_id = surface.map_or(0, |o| o.id());
          msg.write_u32(surface_id);
          msg.write_i32(hotspot_x);
          msg.write_i32(hotspot_y);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_pointer.set_cursor({}, {}, {}, {}, )", serial,surface_id,hotspot_x,hotspot_y,);
          }
        }
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 1);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_pointer.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Enter {
            serial: u32,
            surface: u32, 
            surface_x: f32,
            surface_y: f32,
        },
        Leave {
            serial: u32,
            surface: u32, 
        },
        Motion {
            time: u32,
            surface_x: f32,
            surface_y: f32,
        },
        Button {
            serial: u32,
            time: u32,
            button: u32,
            state: u32,
        },
        Axis {
            time: u32,
            axis: u32,
            value: f32,
        },
        Frame,
        AxisSource {
            axis_source: u32,
        },
        AxisStop {
            time: u32,
            axis: u32,
        },
        AxisDiscrete {
            axis: u32,
            discrete: i32,
        },
        AxisValue120 {
            axis: u32,
            value120: i32,
        },
        AxisRelativeDirection {
            axis: u32,
            direction: u32,
        },
    }
    impl Object for WlPointer {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlPointer {
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
                     let surface = parser.get_u32();
                     let surface_x = parser.get_fixed();
                     let surface_y = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.enter({}, {}, {:.2}, {:.2}, )", serial,surface,surface_x,surface_y,);
                      }
                     Self::Event::Enter { serial, surface, surface_x, surface_y,  }
                 }
                 1 => {
                     let serial = parser.get_u32();
                     let surface = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.leave({}, {}, )", serial,surface,);
                      }
                     Self::Event::Leave { serial, surface,  }
                 }
                 2 => {
                     let time = parser.get_u32();
                     let surface_x = parser.get_fixed();
                     let surface_y = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.motion({}, {:.2}, {:.2}, )", time,surface_x,surface_y,);
                      }
                     Self::Event::Motion { time, surface_x, surface_y,  }
                 }
                 3 => {
                     let serial = parser.get_u32();
                     let time = parser.get_u32();
                     let button = parser.get_u32();
                     let state = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.button({}, {}, {}, {}, )", serial,time,button,state,);
                      }
                     Self::Event::Button { serial, time, button, state,  }
                 }
                 4 => {
                     let time = parser.get_u32();
                     let axis = parser.get_u32();
                     let value = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.axis({}, {}, {:.2}, )", time,axis,value,);
                      }
                     Self::Event::Axis { time, axis, value,  }
                 }
                 5 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.frame()", );
                      }
                     Self::Event::Frame
                 }
                 6 => {
                     let axis_source = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.axis_source({}, )", axis_source,);
                      }
                     Self::Event::AxisSource { axis_source,  }
                 }
                 7 => {
                     let time = parser.get_u32();
                     let axis = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.axis_stop({}, {}, )", time,axis,);
                      }
                     Self::Event::AxisStop { time, axis,  }
                 }
                 8 => {
                     let axis = parser.get_u32();
                     let discrete = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.axis_discrete({}, {}, )", axis,discrete,);
                      }
                     Self::Event::AxisDiscrete { axis, discrete,  }
                 }
                 9 => {
                     let axis = parser.get_u32();
                     let value120 = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.axis_value120({}, {}, )", axis,value120,);
                      }
                     Self::Event::AxisValue120 { axis, value120,  }
                 }
                 10 => {
                     let axis = parser.get_u32();
                     let direction = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_pointer.axis_relative_direction({}, {}, )", axis,direction,);
                      }
                     Self::Event::AxisRelativeDirection { axis, direction,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_keyboard {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlKeyboard {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlKeyboard {
        const INTERFACE: &'static str = "wl_keyboard";
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_keyboard.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Keymap {
            format: u32,
            fd: std::os::fd::OwnedFd, 
            size: u32,
        },
        Enter {
            serial: u32,
            surface: u32, 
            keys: &'a [u32],
        },
        Leave {
            serial: u32,
            surface: u32, 
        },
        Key {
            serial: u32,
            time: u32,
            key: u32,
            state: u32,
        },
        Modifiers {
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        },
        RepeatInfo {
            rate: i32,
            delay: i32,
        },
    }
    impl Object for WlKeyboard {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> WlKeyboard {
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
                     let format = parser.get_u32();
                     let fd = conn.get_fd().unwrap();
                     let size = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_keyboard.keymap({}, {:?}, {}, )", format,fd,size,);
                      }
                     Self::Event::Keymap { format, fd, size,  }
                 }
                 1 => {
                     let serial = parser.get_u32();
                     let surface = parser.get_u32();
                     let keys = parser.get_array_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_keyboard.enter({}, {}, {:?}, )", serial,surface,keys,);
                      }
                     Self::Event::Enter { serial, surface, keys,  }
                 }
                 2 => {
                     let serial = parser.get_u32();
                     let surface = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_keyboard.leave({}, {}, )", serial,surface,);
                      }
                     Self::Event::Leave { serial, surface,  }
                 }
                 3 => {
                     let serial = parser.get_u32();
                     let time = parser.get_u32();
                     let key = parser.get_u32();
                     let state = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_keyboard.key({}, {}, {}, {}, )", serial,time,key,state,);
                      }
                     Self::Event::Key { serial, time, key, state,  }
                 }
                 4 => {
                     let serial = parser.get_u32();
                     let mods_depressed = parser.get_u32();
                     let mods_latched = parser.get_u32();
                     let mods_locked = parser.get_u32();
                     let group = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_keyboard.modifiers({}, {}, {}, {}, {}, )", serial,mods_depressed,mods_latched,mods_locked,group,);
                      }
                     Self::Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group,  }
                 }
                 5 => {
                     let rate = parser.get_i32();
                     let delay = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_keyboard.repeat_info({}, {}, )", rate,delay,);
                      }
                     Self::Event::RepeatInfo { rate, delay,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_touch {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlTouch {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlTouch {
        const INTERFACE: &'static str = "wl_touch";
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_touch.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
        Down {
            serial: u32,
            time: u32,
            surface: u32, 
            id: i32,
            x: f32,
            y: f32,
        },
        Up {
            serial: u32,
            time: u32,
            id: i32,
        },
        Motion {
            time: u32,
            id: i32,
            x: f32,
            y: f32,
        },
        Frame,
        Cancel,
        Shape {
            id: i32,
            major: f32,
            minor: f32,
        },
        Orientation {
            id: i32,
            orientation: f32,
        },
    }
    impl Object for WlTouch {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlTouch {
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
                     let time = parser.get_u32();
                     let surface = parser.get_u32();
                     let id = parser.get_i32();
                     let x = parser.get_fixed();
                     let y = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.down({}, {}, {}, {}, {:.2}, {:.2}, )", serial,time,surface,id,x,y,);
                      }
                     Self::Event::Down { serial, time, surface, id, x, y,  }
                 }
                 1 => {
                     let serial = parser.get_u32();
                     let time = parser.get_u32();
                     let id = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.up({}, {}, {}, )", serial,time,id,);
                      }
                     Self::Event::Up { serial, time, id,  }
                 }
                 2 => {
                     let time = parser.get_u32();
                     let id = parser.get_i32();
                     let x = parser.get_fixed();
                     let y = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.motion({}, {}, {:.2}, {:.2}, )", time,id,x,y,);
                      }
                     Self::Event::Motion { time, id, x, y,  }
                 }
                 3 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.frame()", );
                      }
                     Self::Event::Frame
                 }
                 4 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.cancel()", );
                      }
                     Self::Event::Cancel
                 }
                 5 => {
                     let id = parser.get_i32();
                     let major = parser.get_fixed();
                     let minor = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.shape({}, {:.2}, {:.2}, )", id,major,minor,);
                      }
                     Self::Event::Shape { id, major, minor,  }
                 }
                 6 => {
                     let id = parser.get_i32();
                     let orientation = parser.get_fixed();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_touch.orientation({}, {:.2}, )", id,orientation,);
                      }
                     Self::Event::Orientation { id, orientation,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_output {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlOutput {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlOutput {
        const INTERFACE: &'static str = "wl_output";
        pub fn release(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_output.release()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event<'a> {
        Geometry {
            x: i32,
            y: i32,
            physical_width: i32,
            physical_height: i32,
            subpixel: u32,
            make: &'a str,
            model: &'a str,
            transform: u32,
        },
        Mode {
            flags: u32,
            width: i32,
            height: i32,
            refresh: i32,
        },
        Done,
        Scale {
            factor: i32,
        },
        Name {
            name: &'a str,
        },
        Description {
            description: &'a str,
        },
    }
    impl Object for WlOutput {
        type Event<'a> = Event<'a>;
        fn from_id(id: u32) -> WlOutput {
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
                     let physical_width = parser.get_i32();
                     let physical_height = parser.get_i32();
                     let subpixel = parser.get_u32();
                     let make = parser.get_string();
                     let model = parser.get_string();
                     let transform = parser.get_u32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_output.geometry({}, {}, {}, {}, {}, {}, {}, {}, )", x,y,physical_width,physical_height,subpixel,make,model,transform,);
                      }
                     Self::Event::Geometry { x, y, physical_width, physical_height, subpixel, make, model, transform,  }
                 }
                 1 => {
                     let flags = parser.get_u32();
                     let width = parser.get_i32();
                     let height = parser.get_i32();
                     let refresh = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_output.mode({}, {}, {}, {}, )", flags,width,height,refresh,);
                      }
                     Self::Event::Mode { flags, width, height, refresh,  }
                 }
                 2 => {
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_output.done()", );
                      }
                     Self::Event::Done
                 }
                 3 => {
                     let factor = parser.get_i32();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_output.scale({}, )", factor,);
                      }
                     Self::Event::Scale { factor,  }
                 }
                 4 => {
                     let name = parser.get_string();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_output.name({}, )", name,);
                      }
                     Self::Event::Name { name,  }
                 }
                 5 => {
                     let description = parser.get_string();
                     if *crate::connection::DEBUG {
                          crate::log!(WAYLAND, "==> wl_output.description({}, )", description,);
                      }
                     Self::Event::Description { description,  }
                 }
                _ => unreachable!()
            }
        }
    }
}
pub mod wl_region {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlRegion {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlRegion {
        const INTERFACE: &'static str = "wl_region";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_region.destroy()", );
          }
        }
        pub fn add(&self, conn: &Connection, x: i32, y: i32, width: i32, height: i32, ) {
          let mut msg = Message::<24>::new(self.id, 1);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_region.add({}, {}, {}, {}, )", x,y,width,height,);
          }
        }
        pub fn subtract(&self, conn: &Connection, x: i32, y: i32, width: i32, height: i32, ) {
          let mut msg = Message::<24>::new(self.id, 2);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.write_i32(width);
          msg.write_i32(height);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_region.subtract({}, {}, {}, {}, )", x,y,width,height,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlRegion {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlRegion {
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
pub mod wl_subcompositor {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlSubcompositor {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlSubcompositor {
        const INTERFACE: &'static str = "wl_subcompositor";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subcompositor.destroy()", );
          }
        }
        pub fn get_subsurface(&self, conn: &Connection, surface: &wl_surface::WlSurface, parent: &wl_surface::WlSurface, ) -> wl_subsurface::WlSubsurface {
          let mut msg = Message::<20>::new(self.id, 1);
          let new_id = conn.new_id();
          msg.write_u32(new_id);
          let surface_id = surface.id();
          msg.write_u32(surface_id);
          let parent_id = parent.id();
          msg.write_u32(parent_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subcompositor.get_subsurface({}, {}, {}, )", new_id,surface_id,parent_id,);
          }
          Object::from_id(new_id)
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlSubcompositor {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlSubcompositor {
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
pub mod wl_subsurface {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlSubsurface {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlSubsurface {
        const INTERFACE: &'static str = "wl_subsurface";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subsurface.destroy()", );
          }
        }
        pub fn set_position(&self, conn: &Connection, x: i32, y: i32, ) {
          let mut msg = Message::<16>::new(self.id, 1);
          msg.write_i32(x);
          msg.write_i32(y);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subsurface.set_position({}, {}, )", x,y,);
          }
        }
        pub fn place_above(&self, conn: &Connection, sibling: &wl_surface::WlSurface, ) {
          let mut msg = Message::<12>::new(self.id, 2);
          let sibling_id = sibling.id();
          msg.write_u32(sibling_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subsurface.place_above({}, )", sibling_id,);
          }
        }
        pub fn place_below(&self, conn: &Connection, sibling: &wl_surface::WlSurface, ) {
          let mut msg = Message::<12>::new(self.id, 3);
          let sibling_id = sibling.id();
          msg.write_u32(sibling_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subsurface.place_below({}, )", sibling_id,);
          }
        }
        pub fn set_sync(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 4);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subsurface.set_sync()", );
          }
        }
        pub fn set_desync(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 5);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_subsurface.set_desync()", );
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlSubsurface {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlSubsurface {
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
pub mod wl_fixes {
    use crate::connection::Connection;
    use crate::events::*;
    use crate::connection::Object;
    use crate::protocols::wayland::*;
    use super::*;

    #[derive(Debug)]
    pub struct WlFixes {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlFixes {
        const INTERFACE: &'static str = "wl_fixes";
        pub fn destroy(&self, conn: &Connection, ) {
          let mut msg = Message::<8>::new(self.id, 0);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_fixes.destroy()", );
          }
        }
        pub fn destroy_registry(&self, conn: &Connection, registry: &wl_registry::WlRegistry, ) {
          let mut msg = Message::<12>::new(self.id, 1);
          let registry_id = registry.id();
          msg.write_u32(registry_id);
          msg.build();
          conn.write_request(msg);
          if *crate::connection::DEBUG {
              crate::log!(WAYLAND, "wl_fixes.destroy_registry({}, )", registry_id,);
          }
        }
    }
    #[derive(Debug)]
    pub enum Event {
    }
    impl Object for WlFixes {
        type Event<'a> = Event;
        fn from_id(id: u32) -> WlFixes {
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

