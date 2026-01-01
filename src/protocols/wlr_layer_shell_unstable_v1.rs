use crate::connection::Connection;
use crate::events::*;
use crate::protocols::core::*;
use crate::protocols::impl_obj_prox;

pub use crate::protocols::Object;

impl_obj_prox!(WlrLayerShell, "zwlr_layer_shell_v1");
impl_obj_prox!(WlrLayerSurface, "zwlr_layer_surface_v1");

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WlrLayer {
    Background = 0,
    Bottom,
    Top,
    Overlay,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WlrLayerKeyboard {
    None = 0,
    Exclusive,
    OnDemand,
}

impl WlrLayerShell {
    pub(crate) const GET_LAYER_SURFACE_OP: u16 = 0;
    pub(crate) const DESTROY_OP: u16 = 1;

    pub fn get_layer_surface(
        &self, conn: &Connection, surface: &WlSurface, output: Option<&WlOutput>, layer: WlrLayer,
        namespace: &str,
    ) -> WlrLayerSurface {
        let id = conn.new_id();
        let mut msg = Message::<72>::new(self.id, Self::GET_LAYER_SURFACE_OP);
        let output_id = output.map_or(0, |o| o.id);
        msg.write_u32(id)
            .write_u32(surface.id)
            .write_u32(output_id)
            .write_u32(layer as u32)
            .write_str(namespace)
            .build();
        conn.write_request(msg);

        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_layer_surface(new_id: {}, surface: {}, output: {}, layer: {}, namespace: {})",
                self.interface, self.id, id, surface.id, output_id, layer as u32, namespace,
            );
        }
        Object::from_id(id)
    }

    pub fn destroy(&self, conn: &Connection) {
        let msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg);

        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
                self.interface, self.id
            );
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum WlrLayerSurfaceEvent {
    Configure {
        serial: u32,
        width:  u32,
        height: u32,
    },
    Closed,
}

impl WlrLayerSurface {
    pub const ANCHOR_TOP: u32 = 1;
    pub const ANCHOR_BOTTOM: u32 = 2;
    pub const ANCHOR_LEFT: u32 = 4;
    pub const ANCHOR_RIGHT: u32 = 8;

    pub(crate) const SET_SIZE_OP: u16 = 0;
    pub(crate) const SET_ANCHOR_OP: u16 = 1;
    pub(crate) const SET_EXCLUSIVE_ZONE: u16 = 2;
    pub(crate) const SET_MARGIN_OP: u16 = 3;
    pub(crate) const SET_KEYBOARD_INTERACTIVITY_OP: u16 = 4;
    pub(crate) const GET_POPUP_OP: u16 = 5;
    pub(crate) const ACK_CONFIGURE_OP: u16 = 6;
    pub(crate) const DESTROY_OP: u16 = 7;
    pub(crate) const SET_LAYER_OP: u16 = 8;
    pub(crate) const SET_EXCLUSIVE_EDGE_OP: u16 = 9;

    pub(crate) const CONFIGURE_OP: u16 = 0;
    pub(crate) const CLOSED_OP: u16 = 1;

    pub fn set_size(&self, conn: &Connection, w: u32, h: u32) {
        let mut msg = Message::<16>::new(self.id, Self::SET_SIZE_OP);
        msg.write_u32(w).write_u32(h).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_size(w: {}, h: {})",
                self.interface, self.id, w, h
            );
        }
    }

    pub fn set_anchor(&self, conn: &Connection, anchor: u32) {
        let mut msg = Message::<12>::new(self.id, Self::SET_ANCHOR_OP);
        msg.write_u32(anchor).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_anchor(anchor: {})",
                self.interface, self.id, anchor
            );
        }
    }

    pub fn set_exclusive_zone(&self, conn: &Connection, zone: i32) {
        let mut msg = Message::<12>::new(self.id, Self::SET_EXCLUSIVE_ZONE);
        msg.write_i32(zone).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_exclusive_zone(zone: {})",
                self.interface, self.id, zone
            );
        }
    }

    pub fn set_margin(&self, conn: &Connection, top: i32, right: i32, bottom: i32, left: i32) {
        let mut msg = Message::<24>::new(self.id, Self::SET_MARGIN_OP);
        msg.write_i32(top)
            .write_i32(right)
            .write_i32(bottom)
            .write_i32(left)
            .build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_margin(t: {}, r: {}, b: {}, l: {})",
                self.interface, self.id, top, right, bottom, left
            );
        }
    }

    pub fn set_keyboard_interactivity(
        &self, conn: &Connection, keyboard_interactivity: WlrLayerKeyboard,
    ) {
        let mut msg = Message::<12>::new(self.id, Self::SET_KEYBOARD_INTERACTIVITY_OP);
        msg.write_u32(keyboard_interactivity as u32).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_keyboard_interactivity(keyboard_interactivity: {})",
                self.interface, self.id, keyboard_interactivity as u32
            );
        }
    }

    // TODO
    pub fn get_popup(&self, conn: &Connection, keyboard_interactivity: WlrLayerKeyboard) {
        todo!();
    }

    pub fn ack_configure(&self, conn: &Connection, serial: u32) {
        let mut msg = Message::<12>::new(self.id, Self::ACK_CONFIGURE_OP);
        msg.write_u32(serial).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.ack_configure(serial: {})",
                self.interface, self.id, serial
            );
        }
    }

    pub fn destroy(&self, conn: &Connection, serial: u32) {
        let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
                self.interface, self.id
            );
        }
    }

    pub fn set_layer(&self, conn: &Connection, layer: WlrLayer) {
        let mut msg = Message::<12>::new(self.id, Self::SET_LAYER_OP);
        msg.write_u32(layer as u32).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_layer(layer: {})",
                self.interface, self.id, layer as u32
            );
        }
    }

    pub fn set_exclusive_edge(&self, conn: &Connection, edge: u32) {
        let mut msg = Message::<12>::new(self.id, Self::SET_EXCLUSIVE_EDGE_OP);
        msg.write_u32(edge).build();
        conn.write_request(msg);
        if *crate::connection::DEBUG {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_exclusive_edge(edge: {})",
                self.interface, self.id, edge
            );
        }
    }

    pub fn parse_event(&self, event: Event<'_>) -> WlrLayerSurfaceEvent {
        let parser = event.parser();
        match event.header.opcode {
            Self::CONFIGURE_OP => {
                let serial = parser.get_u32();
                let width = parser.get_u32();
                let height = parser.get_u32();
                if *crate::connection::DEBUG {
                    eprintln!(
                        "[\x1b[32mDEBUG\x1b[0m]: {}#{}.configure(serial: {}, w: {}, h: {})",
                        self.interface, self.id, serial, width, height
                    );
                }
                WlrLayerSurfaceEvent::Configure {
                    serial,
                    width,
                    height,
                }
            }
            Self::CLOSED_OP => {
                if *crate::connection::DEBUG {
                    eprintln!(
                        "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.closed()",
                        self.interface, self.id
                    );
                }
                WlrLayerSurfaceEvent::Closed
            }
            _ => unreachable!(),
        }
    }
}
