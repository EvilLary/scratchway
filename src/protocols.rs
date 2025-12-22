#![allow(unused)]

pub mod wp_single_pixel_buffer_manager_v1 {
    use crate::connection::Connection;
    use crate::core::Object;
    use crate::core::impl_obj_prox;
    use crate::events::*;

    impl_obj_prox!(WpSinglePixelBufferMgr, "wp_single_pixel_buffer_manager_v1");

    impl WpSinglePixelBufferMgr {
        pub(crate) const DESTROY_OP: u16 = 0;
        pub(crate) const CREATE_BUFFER_OP: u16 = 1;

        pub fn destroy(&self, conn: &Connection) {
            let msg = Message::<8>::new(self.id, Self::DESTROY_OP);
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
                self.interface, self.id
            );
            conn.write_request(msg.data());
        }

        pub fn create_buffer(
            &self, conn: &Connection, r: u32, g: u32, b: u32, a: u32,
        ) -> crate::core::WlBuffer {
            let id = conn.new_id();
            let mut msg = Message::<28>::new(self.id, Self::CREATE_BUFFER_OP);
            msg.write_u32(id)
                .write_u32(r)
                .write_u32(g)
                .write_u32(b)
                .write_u32(a)
                .build();
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.create_buffer(new_id: {}, r: {}, g: {}, b: {}, a: {})",
                self.interface, self.id, id, r, g, b, a
            );
            conn.write_request(msg.data());
            Object::from_id(id)
        }
    }
}

pub mod xdg_shell {
    use crate::connection::Connection;
    use crate::core::Object;
    use crate::core::impl_obj_prox;
    use crate::events::*;

    impl_obj_prox!(XdgWmBase, "xdg_wm_base");

    #[derive(Debug)]
    pub enum XdgWmBaseEvent {
        Ping { serial: u32 },
    }

    impl XdgWmBase {
        pub(crate) const DESTROY_OP: u16 = 0;
        pub(crate) const CREATE_POSITIONER_OP: u16 = 1;
        pub(crate) const GET_XDGSURFACE_OP: u16 = 2;
        pub(crate) const PONG_OP: u16 = 3;

        pub(crate) const PING_OP: u16 = 0;

        pub fn destroy(&self, conn: &Connection) {
            let msg = Message::<8>::new(self.id, Self::DESTROY_OP);
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
                self.interface, self.id
            );
            conn.write_request(msg.data());
        }

        pub fn get_xdg_surface(
            &self, conn: &Connection, wl_surface: &crate::core::WlSurface,
        ) -> XdgSurface {
            let id = conn.new_id();
            let mut msg = Message::<16>::new(self.id, Self::GET_XDGSURFACE_OP);
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_xdg_surface(new_id: {}, wl_surface: {})",
                self.interface, self.id, id, wl_surface.id
            );
            msg.write_u32(id).write_u32(wl_surface.id).build();
            conn.write_request(msg.data());
            Object::from_id(id)
        }

        pub fn create_positioner(&self, conn: &Connection) {
            let id = conn.new_id();
            let mut msg = Message::<16>::new(self.id, Self::CREATE_POSITIONER_OP);
            todo!();
            msg.write_u32(id).build();
            // Object::from_id(id);
        }

        pub fn pong(&self, conn: &Connection, serial: u32) {
            let mut msg = Message::<12>::new(self.id, Self::PONG_OP);
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.pong(serial: {})",
                self.interface, self.id, serial
            );
            msg.write_u32(serial).build();
            conn.write_request(msg.data());
        }

        pub fn parse_event(&self, event: Event<'_>) -> XdgWmBaseEvent {
            if event.header.opcode == Self::PING_OP {
                let serial = event.parser().get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.ping(serial: {})",
                    self.interface, self.id, serial
                );
                return XdgWmBaseEvent::Ping { serial };
            }
            unreachable!()
        }
    }

    impl_obj_prox!(XdgSurface, "xdg_surface");

    #[derive(Debug, Copy, Clone)]
    pub enum XdgSurfaceEvent {
        Configure { serial: u32 },
    }
    impl XdgSurface {
        pub(crate) const DESTROY_OP: u16 = 0;
        pub(crate) const GET_TOPLEVEL_OP: u16 = 1;
        pub(crate) const GET_POPUP_OP: u16 = 2;
        pub(crate) const SET_WINDOW_GEOMETRY_OP: u16 = 3;
        pub(crate) const ACK_CONFIGURE_OP: u16 = 4;

        pub(crate) const CONFIGURE_OP: u16 = 0;

        pub fn parse_event(&self, event: Event<'_>) -> XdgSurfaceEvent {
            let parser = event.parser();
            if event.header.opcode == Self::CONFIGURE_OP {
                let serial = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: {}#{}.configure(serial: {})",
                    self.interface, self.id, serial
                );
                return { XdgSurfaceEvent::Configure { serial } };
            }
            unreachable!()
        }

        pub fn destroy(&self, conn: &Connection) {
            let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
                self.interface, self.id
            );
            conn.write_request(msg.data());
        }

        pub fn ack_configure(&self, conn: &Connection, serial: u32) {
            let mut msg = Message::<12>::new(self.id, Self::ACK_CONFIGURE_OP);
            msg.write_u32(serial).build();
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.ack_configure(serial: {})",
                self.interface, self.id, serial
            );
            conn.write_request(msg.data());
        }

        pub fn get_toplevel(&self, conn: &Connection) -> XdgToplevel {
            let id = conn.new_id();
            let mut msg = Message::<12>::new(self.id, Self::GET_TOPLEVEL_OP);
            msg.write_u32(id).build();
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_toplevel(new_id: {})",
                self.interface, self.id, id
            );
            conn.write_request(msg.data());
            Object::from_id(id)
        }
    }

    impl_obj_prox!(XdgToplevel, "xdg_toplevel");

    #[derive(Debug)]
    pub enum XdgToplevelEvent<'a> {
        Configure {
            width: i32,
            height: i32,
            states: &'a [u32],
        },
        ConfigureBounds {
            width: i32,
            height: i32,
        },
        Close,
        WmCapabilities {
            capabilities: &'a [u32],
        },
    }

    impl XdgToplevel {
        pub(crate) const DESTROY_OP: u16 = 0;
        pub(crate) const SET_PARENT_OP: u16 = 1;
        pub(crate) const SET_TITLE_OP: u16 = 2;
        pub(crate) const SET_APP_ID_OP: u16 = 3;

        pub(crate) const CONFIGURE_OP: u16 = 0;
        pub(crate) const CLOSE_OP: u16 = 1;
        pub(crate) const CONFIGURE_BOUNDS_OP: u16 = 2;
        pub(crate) const WM_CAPABILITIES_OP: u16 = 3;

        pub fn parse_event(&self, event: Event<'_>) -> XdgToplevelEvent<'_> {
            let mut parser = event.parser();
            match event.header.opcode {
                Self::CONFIGURE_OP => {
                    let width = parser.get_i32();
                    let height = parser.get_i32();
                    let states = parser.get_array_u32();
                    eprintln!(
                        "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.configure(width: {}, height: {}, states: {:?})",
                        self.interface, self.id, width, height, states
                    );
                    XdgToplevelEvent::Configure {
                        width,
                        height,
                        states,
                    }
                }
                Self::CONFIGURE_BOUNDS_OP => {
                    let width = parser.get_i32();
                    let height = parser.get_i32();
                    eprintln!(
                        "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.configure_bounds(width: {}, height: {})",
                        self.interface, self.id, width, height
                    );
                    XdgToplevelEvent::ConfigureBounds { width, height }
                }
                Self::CLOSE_OP => {
                    eprintln!(
                        "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.close()",
                        self.interface, self.id
                    );
                    XdgToplevelEvent::Close
                }
                Self::WM_CAPABILITIES_OP => {
                    let capabilities = parser.get_array_u32();
                    eprintln!(
                        "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.wm_capabilities(capabilities: {:?})",
                        self.interface, self.id, capabilities
                    );
                    XdgToplevelEvent::WmCapabilities { capabilities }
                }
                _ => unreachable!(),
            }
            // todo!()
        }

        pub fn destroy(&self, conn: &Connection) {
            let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
                self.interface, self.id
            );
            conn.write_request(msg.data());
        }

        pub fn set_title(&self, conn: &Connection, title: impl AsRef<str>) {
            let mut msg = Message::<64>::new(self.id, Self::SET_TITLE_OP);
            msg.write_str(title.as_ref()).build();
            conn.write_request(msg.data());
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_title(title: {})",
                self.interface,
                self.id,
                title.as_ref()
            );
        }

        pub fn set_app_id(&self, conn: &Connection, app_id: impl AsRef<str>) {
            let mut msg = Message::<64>::new(self.id, Self::SET_APP_ID_OP);
            msg.write_str(app_id.as_ref()).build();
            conn.write_request(msg.data());
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_title(app_id: {})",
                self.interface,
                self.id,
                app_id.as_ref()
            );
        }
    }
}
