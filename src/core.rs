#![allow(unused)]

pub mod wl_display {
    use crate::core::wl_registry::WlRegistry;
    use crate::{connection::Connection, events::*};

    #[derive(Debug)]
    pub struct WlDisplay {
        pub id: u32,
    }

    #[derive(Debug)]
    pub enum WlDisplayEvent<'a> {
        Error {
            object_id: u32,
            code: u32,
            message: &'a str,
        },
        DeleteId {
            id: u32,
        },
    }

    impl WlDisplay {
        pub(crate) fn new(id: u32) -> Self {
            Self { id }
        }

        pub(crate) const GET_REGISTRY_OP: u16 = 1;
        pub(crate) const SYNC_OP: u16 = 1;

        pub fn get_registry(&self, connection: &mut Connection) -> WlRegistry {
            let id = connection.new_id();
            let mut msg = Message::<12>::new(self.id, WlDisplay::GET_REGISTRY_OP);
            msg.write_u32(id).build();
            connection.write_request(msg);
            WlRegistry::new(id)
        }

        pub fn sync(&self, connection: &mut Connection) {
            let id = connection.new_id();
            let mut msg = Message::<12>::new(self.id, 0);
            msg.write_u32(id).build();
            connection.write_request(msg);
        }

        pub fn parse_event<'a, 's>(&'s self, event: Event<'a>) -> WlDisplayEvent<'a> {
            let mut parser = EventDataParser::new(event.data);
            match event.header.opcode {
                0 => {
                    let object_id = parser.get_u32();
                    let code = parser.get_u32();
                    // SAFETY: the reference behind message is valid for as long
                    // as the event.data is valid, Rust just can't know it
                    // let message = parser.get_string();
                    let message = unsafe {
                        let str = parser.get_string();
                        let len = str.len();
                        let ptr = str.as_ptr();
                        let slice = core::slice::from_raw_parts(ptr, len);
                        core::str::from_utf8_unchecked(slice)
                    };
                    WlDisplayEvent::Error {
                        object_id,
                        code,
                        message,
                    }
                }
                1 => WlDisplayEvent::DeleteId {
                    id: parser.get_u32(),
                },
                _ => unreachable!(),
            }
        }
    }
}

pub mod wl_registry {
    use crate::{
        connection::{self, Connection},
        events::*,
    };

    #[derive(Debug)]
    pub struct WlRegistry {
        pub id: u32,
    }

    #[derive(Debug)]
    pub enum WlRegistryEvent<'a> {
        Global {
            name: u32,
            interface: &'a str,
            version: u32,
        },
        GlobalRemove {
            name: u32,
        },
    }

    impl WlRegistry {
        pub(crate) fn new(id: u32) -> Self {
            Self { id }
        }

        pub fn bind(
            &self, connection: &mut Connection, name: u32, interface: &str, version: u32,
        ) -> u32 {
            let mut msg = Message::<64>::new(self.id, 0);
            let id = connection.new_id();
            msg.write_u32(name)
                .write_str(interface)
                .write_u32(version)
                .write_u32(id)
                .build();
            connection.write_request(msg);
            eprintln!(
                "\x1b[32m[DEBUG]\x1b[0m: wl_registry#2.bind(name: {}, interface: {}, version: {})",
                name, interface, version
            );
            id
        }

        pub fn parse_event<'a>(&self, event: Event<'a>) -> WlRegistryEvent<'a> {
            let mut parser = EventDataParser::new(event.data);
            match event.header.opcode {
                0 => {
                    let name = parser.get_u32();
                    let interface = unsafe {
                        let str = parser.get_string();
                        let len = str.len();
                        let ptr = str.as_ptr();
                        let slice = core::slice::from_raw_parts(ptr, len);
                        core::str::from_utf8_unchecked(slice)
                    };
                    let version = parser.get_u32();
                    WlRegistryEvent::Global {
                        name,
                        interface,
                        version,
                    }
                }
                1 => WlRegistryEvent::GlobalRemove {
                    name: parser.get_u32(),
                },
                _ => unreachable!(),
            }
        }
    }
}
