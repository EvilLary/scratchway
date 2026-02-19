#![allow(unused)]
use crate::{Connection, Listener, WlInterface, WlProxy, connection::wire::*, log, objects::Interface};

pub use wl_display::WlDisplay;
pub mod wl_display {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct WlDisplay {
        id: u32,
        version: u32,
    }

    impl PartialEq for WlDisplay {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl ::core::cmp::Eq for WlDisplay {}

    impl Default for WlDisplay {
        fn default() -> Self {
            Self {
                id: 1,
                version: Self::INTERFACE.version,
            }
        }
    }

    impl WlDisplay {
        pub const INTERFACE: &'static Interface = &Interface {
            name: "wl_display",
            version: 1,
            events: &["error", "delete_id"],
            requests: &["sync", "get_registry"],
        };
    }

    impl WlDisplay {
        pub(crate) fn sync<S>(&self, conn: &mut Connection<S>) -> WlCallback {
            let callback = conn.deaf_wlinterface(None, 1);
            let mut msg = Message::<12>::new(self.id, 0);
            msg.write_u32(<WlCallback as WlInterface>::id(&callback));
            msg.build();
            conn.write_request(msg.data());
            if conn.debug {
                log!(WAYLAND, "wl_display.sync(new {:?})", callback);
            }
            callback
        }

        // Maybe expand on this approach?
        pub(crate) fn get_registry_deaf<S>(&self, conn: &mut Connection<S>) -> wl_registry::WlRegistry {
            let new_object = conn.deaf_wlinterface(None, self.version);
            let mut msg = Message::<12>::new(self.id, 1u16);
            msg.write_u32(<wl_registry::WlRegistry as WlInterface>::id(&new_object));
            msg.build();
            conn.write_request(msg.data());
            if conn.debug {
                log!(WAYLAND, "wl_display.get_registry(new {:?})", new_object);
            }
            new_object
        }

        pub(crate) fn get_registry<S>(&self, conn: &mut Connection<S>) -> wl_registry::WlRegistry
        where
            S: Listener<WlRegistry>,
        {
            let new_object = conn.create_new_object(None, self.version);
            let mut msg = Message::<12>::new(self.id, 1u16);
            msg.write_u32(<wl_registry::WlRegistry as WlInterface>::id(&new_object));
            msg.build();
            conn.write_request(msg.data());
            if conn.debug {
                log!(WAYLAND, "wl_display.get_registry(new {:?})", new_object);
            }
            new_object
        }
    }

    impl ::std::fmt::Debug for WlDisplay {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", WlDisplay::INTERFACE.name, self.id))
        }
    }

    impl ::std::fmt::Display for WlDisplay {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", WlDisplay::INTERFACE.name, self.id))
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

    impl WlInterface for WlDisplay {
        type Event<'a> = Event<'a>;

        fn from_proxy(wl_proxy: WlProxy) -> Self {
            Self {
                id: wl_proxy.id,
                version: wl_proxy.version,
            }
        }

        fn id(&self) -> u32 {
            self.id
        }

        fn version(&self) -> u32 {
            self.version
        }

        fn parse_event<'a, S>(&self, conn: &mut Connection<S>, event: &'a WlEvent) -> Event<'a> {
            let parser = event.parser();
            match event.header.opcode {
                0 => {
                    let object_id = parser.get_u32();
                    let code = parser.get_u32();
                    let message = parser.get_string();
                    // log!(
                    //     WAYLAND,
                    //     conn.debug,
                    //     "==> wl_display.error({}, {}, {})",
                    //     object_id,
                    //     code,
                    //     message
                    // );
                    Event::Error {
                        object_id,
                        code,
                        message,
                    }
                },
                1 => {
                    let id = parser.get_u32();
                    // if conn.debug {
                    //     log!(WAYLAND, "==> wl_display.delete_id({})", id);
                    // }
                    Event::DeleteId { id }
                },
                _ => unreachable!(),
            }
        }

        fn interface() -> &'static Interface {
            Self::INTERFACE
        }
    }
}

pub use wl_registry::WlRegistry;
pub mod wl_registry {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct WlRegistry {
        id: u32,
        version: u32,
    }

    impl WlRegistry {
        pub const INTERFACE: &'static Interface = &Interface {
            name: "wl_registry",
            version: 1,
            events: &["global", "global_remove"],
            requests: &["bind"],
        };

        pub fn bind<I, S>(&self, conn: &mut Connection<S>, name: u32, version: u32) -> I
        where
            I: WlInterface,
            S: Listener<I>,
        {
            let new_object = conn.create_new_object::<I>(None, version);
            let mut msg = Message::<96>::new(self.id, 0);
            msg.write_u32(name);
            msg.write_string(I::interface().name);
            msg.write_u32(version);
            msg.write_u32(<I as WlInterface>::id(&new_object));
            msg.build();
            conn.write_request(msg.data());
            if conn.debug {
                log!(
                    WAYLAND,
                    "{:?}.bind(new {:?}, {}, {})",
                    self,
                    new_object,
                    name,
                    version
                );
            }
            new_object
        }
    }

    impl ::std::fmt::Debug for WlRegistry {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", WlRegistry::INTERFACE.name, self.id))
        }
    }

    impl ::std::fmt::Display for WlRegistry {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{}#{}", WlRegistry::INTERFACE.name, self.id))
        }
    }

    impl PartialEq for WlRegistry {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl ::core::cmp::Eq for WlRegistry {}

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

    impl WlInterface for WlRegistry {
        type Event<'a> = Event<'a>;

        fn interface() -> &'static Interface {
            Self::INTERFACE
        }

        fn from_proxy(wl_proxy: WlProxy) -> Self {
            Self {
                id: wl_proxy.id,
                version: wl_proxy.version,
            }
        }

        fn id(&self) -> u32 {
            self.id
        }

        fn version(&self) -> u32 {
            self.version
        }

        fn parse_event<'a, S>(&self, conn: &mut Connection<S>, event: &'a WlEvent) -> Self::Event<'a> {
            let parser = event.parser();
            match event.header.opcode {
                0 => {
                    let name = parser.get_u32();
                    let interface = parser.get_string();
                    let version = parser.get_u32();
                    // log!(
                    //     WAYLAND,
                    //     conn.debug,
                    //     "==> {:?}.global({}, {}, {})",
                    //     self,
                    //     name,
                    //     interface,
                    //     version
                    // );
                    Self::Event::Global {
                        name,
                        interface,
                        version,
                    }
                },
                1 => {
                    let name = parser.get_u32();
                    // log!(WAYLAND, conn.debug, "==> {:?}.global_remove({})", self, name);
                    Self::Event::GlobalRemove { name }
                },
                _ => unreachable!(),
            }
        }
    }
}

scr_scanner::generate!("./protocols/wayland.xml");
