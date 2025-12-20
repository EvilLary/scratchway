#![allow(unused)]

use crate::{
    connection::{self, Connection},
    events::*,
};

pub trait Object {
    fn from_id(id: u32) -> Self;
}

#[derive(Debug)]
pub struct WlDisplay {
    pub id: u32,
    pub interface: &'static str,
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
    pub fn new(id: u32) -> Self {
        Self {
            id,
            interface: "wl_display",
        }
    }

    pub(crate) const SYNC_OP: u16 = 0;
    pub(crate) const GET_REGISTRY_OP: u16 = 1;

    pub(crate) const ERROR_OP: u16 = 0;
    pub(crate) const DELETE_ID_OP: u16 = 1;

    pub fn get_registry(&self, conn: &Connection) -> WlRegistry {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_REGISTRY_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        WlRegistry::from_id(id)
    }

    pub fn sync(&self, conn: &Connection) {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, 0);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
    }

    pub fn parse_event<'a, 's>(&'s self, event: Event<'a>) -> WlDisplayEvent<'a> {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            Self::ERROR_OP => {
                let object_id = parser.get_u32();
                let code = parser.get_u32();
                let message = parser.get_string();
                WlDisplayEvent::Error {
                    object_id,
                    code,
                    message,
                }
            }
            Self::DELETE_ID_OP => WlDisplayEvent::DeleteId {
                id: parser.get_u32(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct WlRegistry {
    pub id: u32,
    pub interface: &'static str,
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

impl Object for WlRegistry {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_registry",
        }
    }
}

impl WlRegistry {
    pub(crate) const BIND_OP: u16 = 0;

    pub(crate) const GLOBAL_OP: u16 = 0;
    pub(crate) const GLOBAL_REMOVE_OP: u16 = 1;

    pub fn bind<O: Object>(
        &self, conn: &Connection, name: u32, interface: &str, version: u32,
    ) -> O {
        let mut msg = Message::<64>::new(self.id, Self::BIND_OP);
        let id = conn.new_id();
        msg.write_u32(name)
            .write_str(interface)
            .write_u32(version)
            .write_u32(id)
            .build();
        conn.write_request(msg.data());
        eprintln!(
            "\x1b[32m[DEBUG]\x1b[0m: wl_registry#2.bind(name: {}, interface: {}, version: {})",
            name, interface, version
        );
        <O as Object>::from_id(id)
    }

    pub fn parse_event<'a>(&self, event: Event<'a>) -> WlRegistryEvent<'a> {
        let mut parser = EventDataParser::new(event.data);
        match event.header.opcode {
            Self::GLOBAL_OP => {
                let name = parser.get_u32();
                let interface = parser.get_string();
                let version = parser.get_u32();
                WlRegistryEvent::Global {
                    name,
                    interface,
                    version,
                }
            }
            Self::GLOBAL_REMOVE_OP => WlRegistryEvent::GlobalRemove {
                name: parser.get_u32(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct WlSeat {
    pub id: u32,
    pub interface: &'static str,
}

impl Object for WlSeat {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_seat",
        }
    }
}

#[derive(Debug)]
pub enum WlSeatEvent<'a> {
    Capabilities { capabilities: u32 },
    Name { name: &'a str },
}

impl WlSeat {
    pub(crate) const GET_POINTER_OP: u16 = 0;
    pub(crate) const GET_KEYBOARD_OP: u16 = 1;
    pub(crate) const GET_TOUCH_OP: u16 = 2;
    pub(crate) const RELEASE_OP: u16 = 3;

    pub(crate) const NAME_OP: u16 = 0;
    pub(crate) const CAPABILITIES_OP: u16 = 1;

    pub fn parse_event(&self, event: Event<'_>) -> WlSeatEvent<'_> {
        let parser = event.parser();
        match event.header.opcode {
            Self::NAME_OP => WlSeatEvent::Name {
                name: parser.get_string(),
            },
            Self::CAPABILITIES_OP => WlSeatEvent::Capabilities {
                capabilities: parser.get_u32(),
            },
            _ => unreachable!(),
        }
    }

    pub fn get_pointer(&self, conn: &Connection) -> WlPointer {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_POINTER_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        WlPointer::from_id(id)
    }

    pub fn get_touch(&self, conn: &Connection) -> WlTouch {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_TOUCH_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        WlTouch::from_id(id)
    }

    pub fn get_keyboard(&self, conn: &Connection) -> WlKeyboard {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_KEYBOARD_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        WlKeyboard::from_id(id)
    }

    pub fn release(self, conn: &mut Connection) {
        let mut msg = Message::<8>::new(self.id, Self::RELEASE_OP);
        conn.write_request(msg.data());
    }
}

#[derive(Debug)]
pub struct WlPointer {
    pub id: u32,
    pub interface: &'static str,
}

// TODO
impl WlPointer {}

impl Object for WlPointer {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_pointer",
        }
    }
}

#[derive(Debug)]
pub struct WlKeyboard {
    pub id: u32,
    pub interface: &'static str,
}

pub enum WlKeyboardEvent<'a> {
    Keymap {
        format: u32,
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
        rate: u32,
        delay: u32,
    },
}

impl WlKeyboard {
    pub(crate) const RELEASE_OP: u16 = 0;

    pub(crate) const KEYMAP_OP: u16 = 0;
    pub(crate) const ENTER_OP: u16 = 1;
    pub(crate) const LEAVE_OP: u16 = 2;
    pub(crate) const KEY_OP: u16 = 3;
    pub(crate) const MODIFIERS_OP: u16 = 4;
    pub(crate) const REPEAT_INFO_OP: u16 = 5;

    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            interface: "wl_keyboard",
        }
    }

    pub fn release(self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::RELEASE_OP);
        conn.write_request(msg.data());
    }
    pub fn parse_event(&self, event: Event<'_>) -> WlKeyboardEvent<'_> {
        let parser = event.parser();
        match event.header.opcode {
            Self::KEYMAP_OP => WlKeyboardEvent::Keymap {
                format: parser.get_u32(),
                size: parser.get_u32(),
            },
            Self::ENTER_OP => WlKeyboardEvent::Enter {
                serial: parser.get_u32(),
                surface: parser.get_u32(),
                keys: parser.get_array_u32(),
            },
            Self::LEAVE_OP => WlKeyboardEvent::Leave {
                serial: parser.get_u32(),
                surface: parser.get_u32(),
            },
            Self::KEY_OP => WlKeyboardEvent::Key {
                serial: parser.get_u32(),
                time: parser.get_u32(),
                key: parser.get_u32(),
                state: parser.get_u32(),
            },
            Self::MODIFIERS_OP => WlKeyboardEvent::Modifiers {
                serial: parser.get_u32(),
                mods_depressed: parser.get_u32(),
                mods_latched: parser.get_u32(),
                mods_locked: parser.get_u32(),
                group: parser.get_u32(),
            },
            Self::REPEAT_INFO_OP => WlKeyboardEvent::RepeatInfo {
                rate: parser.get_u32(),
                delay: parser.get_u32(),
            },
            _ => unreachable!(),
        }
    }
}
impl Object for WlKeyboard {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_keyboard",
        }
    }
}

#[derive(Debug)]
pub struct WlTouch {
    pub id: u32,
    pub interface: &'static str,
}

// TODO
impl WlTouch {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            interface: "wl_touch",
        }
    }
}

impl Object for WlTouch {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_touch",
        }
    }
}

#[derive(Debug)]
pub struct WlCompositor {
    id: u32,
    interface: &'static str,
}

impl WlCompositor {
    pub const CREATE_SURFACE_OP: u16 = 0;
    pub const CREATE_REGION_OP: u16 = 1;

    pub fn create_surface(&self, conn: &Connection) -> WlSurface {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::CREATE_SURFACE_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        WlSurface::new(id)
    }

    pub fn create_region(&self, conn: &Connection) -> WlRegion {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::CREATE_REGION_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        WlRegion::new(id)
    }
}

impl Object for WlCompositor {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_compositor",
        }
    }
}

#[derive(Debug)]
pub struct WlSurface {
    id: u32,
    interface: &'static str,
}

impl WlSurface {
    pub(crate) const DESTROY_OP: u16 = 0;
    pub(crate) const ATTACH_OP: u16 = 1;
    pub(crate) const DAMANGE_OP: u16 = 2;
    pub(crate) const FRAME_OP: u16 = 3;
    pub(crate) const SET_OPAQUE_REGION_OP: u16 = 4;
    pub(crate) const SET_INPUT_REGION_OP: u16 = 5;
    pub(crate) const COMMIT_OP: u16 = 6;
    pub(crate) const SET_BUFFER_TRANSFORM_OP: u16 = 7;
    pub(crate) const SET_BUFFER_SCALE_OP: u16 = 8;
    pub(crate) const DAMANGE_BUFFER_OP: u16 = 9;
    pub(crate) const OFFSET_OP: u16 = 10;

    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            interface: "wl_surface",
        }
    }

    pub fn destroy(self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg.data());
    }

    pub fn attach(&self, conn: &Connection, wl_buffer: Option<&WlBuffer>, x: i32, y: i32) {
        let mut msg = Message::<20>::new(self.id, Self::ATTACH_OP);
        msg.write_u32(wl_buffer.map_or(0, |buf| buf.id))
            .write_i32(x)
            .write_i32(y)
            .build();
        conn.write_request(msg.data());
    }

    pub fn damage(&self, conn: &Connection, x: i32, y: i32, w: i32, h: i32) {
        let mut msg = Message::<24>::new(self.id, Self::DAMANGE_OP);
        msg.write_i32(x)
            .write_i32(y)
            .write_i32(w)
            .write_i32(h)
            .build();
        conn.write_request(msg.data());
    }

    // TODO
    // pub fn frame(&self, conn: &Connection, x: i32, y: i32, w: i32, h: i32) {
    //     let mut msg = Message::<8>::new(self.id, Self::DAMANGE_OP);
    //     msg.write_i32(x)
    //         .write_i32(y)
    //         .write_i32(w)
    //         .write_i32(h)
    //         .build();
    //     conn.write_request(msg.data());
    // }

    pub fn set_opaque_region(&self, conn: &Connection, wl_region: Option<WlRegion>) {
        let mut msg = Message::<12>::new(self.id, Self::SET_OPAQUE_REGION_OP);
        msg.write_u32(wl_region.map_or(0, |w| w.id)).build();
        conn.write_request(msg.data());
    }

    pub fn set_input_region(&self, conn: &Connection, wl_region: Option<WlRegion>) {
        let mut msg = Message::<12>::new(self.id, Self::SET_INPUT_REGION_OP);
        msg.write_u32(wl_region.map_or(0, |w| w.id)).build();
        conn.write_request(msg.data());
    }

    pub fn commit(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::COMMIT_OP);
        conn.write_request(msg.data());
    }

    pub fn set_buffer_transform(&self, conn: &Connection, transform: WlOutputTransform) {
        let mut msg = Message::<12>::new(self.id, Self::SET_BUFFER_TRANSFORM_OP);
        msg.write_i32(transform as i32).build();
        conn.write_request(msg.data());
    }

    pub fn set_buffer_scale(&self, conn: &Connection, scale: i32) {
        let mut msg = Message::<12>::new(self.id, Self::SET_BUFFER_SCALE_OP);
        msg.write_i32(scale).build();
        conn.write_request(msg.data());
    }

    pub fn damage_buffer(&self, conn: &Connection, x: i32, y: i32, w: i32, h: i32) {
        let mut msg = Message::<24>::new(self.id, Self::DAMANGE_BUFFER_OP);
        msg.write_i32(x)
            .write_i32(y)
            .write_i32(w)
            .write_i32(h)
            .build();
        conn.write_request(msg.data());
    }

    pub fn offset(&self, conn: &Connection, x: i32, y: i32) {
        let mut msg = Message::<16>::new(self.id, Self::OFFSET_OP);
        msg.write_i32(x).write_i32(y).build();
        conn.write_request(msg.data());
    }
}

impl Object for WlSurface {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_surface",
        }
    }
}

#[derive(Debug)]
pub struct WlRegion {
    pub id: u32,
    pub interface: &'static str,
}

impl WlRegion {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            interface: "wl_region",
        }
    }
}

impl Object for WlRegion {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_region",
        }
    }
}

#[derive(Debug)]
pub struct WlBuffer {
    pub id: u32,
    pub interface: &'static str,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum WlOutputTransform {
    Normal = 0,
    D90,
    D180,
    D270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

impl From<i32> for WlOutputTransform {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::D90,
            2 => Self::D180,
            3 => Self::D270,
            4 => Self::Flipped,
            5 => Self::Flipped90,
            6 => Self::Flipped180,
            7 => Self::Flipped270,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum WlOutputEvent<'a> {
    Geometry {
        x: i32,
        y: i32,
        physical_w: i32,
        physical_h: i32,
        subpixel: WlOutputSubPixel,
        make: &'a str,
        model: &'a str,
        transform: WlOutputTransform,
    },
    Mode {
        flags: WlOutputMode,
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

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum WlOutputMode {
    Current = 1,
    Preffered,
}

impl From<i32> for WlOutputMode {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Current,
            2 => Self::Preffered,
            _ => unreachable!()
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum WlOutputSubPixel {
    Unknown = 0,
    None,
    HorizontalRgb,
    HorizontalBgr,
    VerticalRgb,
    VerticalBgr,
}

impl From<i32> for WlOutputSubPixel {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::HorizontalRgb,
            3 => Self::HorizontalBgr,
            4 => Self::VerticalRgb,
            5 => Self::VerticalBgr,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct WlOutput {
    pub id: u32,
    pub interface: &'static str,
}

impl WlOutput {
    pub(crate) const RELEASE_OP: u16 = 0;

    pub(crate) const GEOMETRY_OP: u16 = 0;
    pub(crate) const MODE_OP: u16 = 1;
    pub(crate) const DONE_OP: u16 = 2;
    pub(crate) const SCALE_OP: u16 = 3;
    pub(crate) const NAME_OP: u16 = 4;
    pub(crate) const DESCRIPTION_OP: u16 = 5;

    pub fn parse_event(&self, event: Event<'_>) -> WlOutputEvent<'_> {
        let parser = event.parser();
        match event.header.opcode {
            Self::GEOMETRY_OP => WlOutputEvent::Geometry {
                x: parser.get_i32(),
                y: parser.get_i32(),
                physical_w: parser.get_i32(),
                physical_h: parser.get_i32(),
                subpixel: WlOutputSubPixel::from(parser.get_i32()),
                make: parser.get_string(),
                model: parser.get_string(),
                transform: WlOutputTransform::from(parser.get_i32()),
            },
            Self::MODE_OP => WlOutputEvent::Mode {
                flags: WlOutputMode::from(parser.get_i32()),
                width: parser.get_i32(),
                height: parser.get_i32(),
                refresh: parser.get_i32(),
            },
            Self::DONE_OP => WlOutputEvent::Done,
            Self::SCALE_OP => WlOutputEvent::Scale {
                factor: parser.get_i32()
            },
            Self::NAME_OP => WlOutputEvent::Name {
                name: parser.get_string()
            },
            Self::DESCRIPTION_OP => WlOutputEvent::Description {
                description: parser.get_string()
            },
            _ => unreachable!(),
        }
    }
}

impl Object for WlOutput {
    fn from_id(id: u32) -> Self {
        Self {
            id,
            interface: "wl_output",
        }
    }
}
