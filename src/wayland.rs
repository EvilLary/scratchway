#![allow(unused)]

use crate::connection::{Reader, WaylandBuffer, Writer};
use crate::events::*;
use crate::prelude::*;
use crate::log;

pub mod wl_display {
    use super::*;

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
        pub fn sync(&self, writer: &WaylandBuffer<Writer>) -> wl_callback::WlCallback {
            let mut msg = Message::<12>::new(self.id, 0);
            let new_id = writer.new_id();
            let new_cb = Object::from_id(new_id);
            msg.write_u32(new_id);
            msg.build();
            writer.write_request(msg.data());
            log!(WAYLAND, "wl_display.sync(new {})", new_cb);
            new_cb
        }
        pub fn get_registry(&self, writer: &WaylandBuffer<Writer>) -> wl_registry::WlRegistry {
            let mut msg = Message::<12>::new(self.id, 1);
            let new_id = writer.new_id();
            let new_ty = Object::from_id(new_id);
            msg.write_u32(new_id);
            msg.build();
            writer.write_request(msg.data());
            log!(WAYLAND, "wl_display.get_registry(new {})", new_ty);
            new_ty
        }
    }
    impl ::std::fmt::Display for WlDisplay {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", Self::INTERFACE, self.id))
        }
    }
    impl ::std::fmt::Debug for WlDisplay {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", Self::INTERFACE, self.id))
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
    pub enum Error {
        InvalidObject = 0,
        InvalidMethod = 1,
        NoMemory = 2,
        Implementation = 3,
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
        fn parse_event<'a>(
            &self, reader: &WaylandBuffer<Reader>, event: WlEvent<'a>,
        ) -> Self::Event<'a> {
            let parser = event.parser();
            match event.header.opcode {
                0 => {
                    let object_id = parser.get_u32();
                    let code = parser.get_u32();
                    let message = parser.get_string();
                    log!(
                        WAYLAND,
                        "==> wl_display.error({}, {}, {})",
                        object_id,
                        code,
                        message
                    );
                    Self::Event::Error {
                        object_id,
                        code,
                        message,
                    }
                }
                1 => {
                    let id = parser.get_u32();
                    log!(WAYLAND, "==> wl_display.delete_id({})", id);
                    Self::Event::DeleteId { id }
                }
                _ => unreachable!(),
            }
        }
    }
}

pub mod wl_registry {
    use super::*;

    pub struct WlRegistry {
        pub(crate) id: u32,
        pub(crate) interface: &'static str,
    }
    impl WlRegistry {
        const INTERFACE: &'static str = "wl_registry";
        pub fn bind<O: Object>(
            &self, writer: &WaylandBuffer<Writer>, name: u32, interface: &str, version: u32,
        ) -> O {
            let mut msg = Message::<64>::new(self.id, 0);
            let new_id = writer.new_id();
            msg.write_u32(name);
            msg.write_string(interface);
            msg.write_u32(version);
            msg.write_u32(new_id);
            msg.build();
            writer.write_request(msg.data());
            log!(
                WAYLAND,
                "{}.bind(new {}#{}, {}, {})",
                self,
                interface,
                new_id,
                name,
                version
            );
            Object::from_id(new_id)
        }
    }
    impl ::std::fmt::Display for WlRegistry {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", Self::INTERFACE, self.id))
        }
    }
    impl ::std::fmt::Debug for WlRegistry {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", Self::INTERFACE, self.id))
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
        fn parse_event<'a>(
            &self, reader: &WaylandBuffer<Reader>, event: WlEvent<'a>,
        ) -> Self::Event<'a> {
            let parser = event.parser();
            match event.header.opcode {
                0 => {
                    let name = parser.get_u32();
                    let interface = parser.get_string();
                    let version = parser.get_u32();
                    log!(
                        WAYLAND,
                        "==> {}.global({}, {}, {})",
                        self,
                        name,
                        interface,
                        version
                    );
                    Self::Event::Global {
                        name,
                        interface,
                        version,
                    }
                }
                1 => {
                    let name = parser.get_u32();
                    log!(
                        WAYLAND,
                        "==> {}.global_remove({})",
                        self,
                        name
                    );
                    Self::Event::GlobalRemove { name }
                }
                _ => unreachable!(),
            }
        }
    }
}

scr_scanner::generate!("./protocols/wayland.xml");
