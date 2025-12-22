#![allow(unused)]

use crate::{
    connection::{self, Connection},
    events::*,
};

pub trait Object {
    type Event;

    fn from_id(id: u32) -> Self;

    #[inline(always)]
    fn id(&self) -> u32;

    fn interface(&self) -> &'static str;

    fn parse_event_obj(&self, event: Event<'_>) -> Self::Event {
        todo!()
    }
}

macro_rules! impl_obj_prox {
    ($prox:ident, $interface:literal) => {
        #[derive(Debug)]
        pub struct $prox {
            pub id: u32,
            pub interface: &'static str,
        }
        impl Object for $prox {
            type Event = u32;
            fn from_id(id: u32) -> Self {
                Self {
                    id,
                    interface: $interface,
                }
            }
            fn id(&self) -> u32 {
                self.id
            }
            fn interface(&self) -> &'static str {
                self.interface
            }
            fn parse_event_obj(&self, event: Event<'_>) -> Self::Event {
                todo!()
            }
        }
        impl PartialEq<u32> for $prox {
            fn eq(&self, other: &u32) -> bool {
                self.id() == *other
            }
        }
        impl<O: Object> PartialEq<O> for $prox {
            fn eq(&self, other: &O) -> bool {
                self.id() == other.id()
            }
        }
    };
}

pub(crate) use impl_obj_prox;

impl_obj_prox!(WlDisplay, "wl_display");

impl Default for WlDisplay {
    fn default() -> Self {
        Object::from_id(1)
    }
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
        if conn.debug {
            eprintln!(
                "[\x1b[32mDEBUG\x1b[0m]: {}#{}.sync(new id: {})",
                self.interface, self.id, id,
            );
        }
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
                eprintln!(
                    "[\x1b[31mERROR\x1b[0m]: ==> {}#{}.error(object_id: {}, code: {}, message: {})",
                    self.interface, self.id, object_id, code, message
                );
                WlDisplayEvent::Error {
                    object_id,
                    code,
                    message,
                }
            }
            Self::DELETE_ID_OP => {
                let id = parser.get_u32();
                // if conn.debug {
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.delete_id(id: {})",
                    self.interface, self.id, id,
                );
                // }
                WlDisplayEvent::DeleteId { id }
            }
            _ => unreachable!(),
        }
    }
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

impl_obj_prox!(WlRegistry, "wl_registry");

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
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.bind(new_id: {}, name: {}, interface: {}, version: {})",
            self.interface, self.id, id, name, interface, version
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
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.global(name: {}, interface: {}, version: {})",
                    self.interface, self.id, name, interface, version
                );
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

impl_obj_prox!(WlSeat, "wl_seat");

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
            Self::NAME_OP => {
                let name = parser.get_string();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.name(name: {})",
                    self.interface, self.id, name
                );
                WlSeatEvent::Name { name }
            }
            Self::CAPABILITIES_OP => {
                let capabilities = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.name(capabilities: {})",
                    self.interface, self.id, capabilities
                );
                WlSeatEvent::Capabilities { capabilities }
            }
            _ => unreachable!(),
        }
    }

    pub fn get_pointer(&self, conn: &Connection) -> WlPointer {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_POINTER_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_pointer(new_id: {})",
            self.interface, self.id, id,
        );
        WlPointer::from_id(id)
    }

    pub fn get_touch(&self, conn: &Connection) -> WlTouch {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_TOUCH_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_touch(new_id: {})",
            self.interface, self.id, id,
        );
        WlTouch::from_id(id)
    }

    pub fn get_keyboard(&self, conn: &Connection) -> WlKeyboard {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::GET_KEYBOARD_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_keyboard(new_id: {})",
            self.interface, self.id, id,
        );
        WlKeyboard::from_id(id)
    }

    pub fn release(&self, conn: &mut Connection) {
        let mut msg = Message::<8>::new(self.id, Self::RELEASE_OP);
        conn.write_request(msg.data());
    }
}

impl_obj_prox!(WlPointer, "wl_pointer");
// TODO
impl WlPointer {}

impl_obj_prox!(WlKeyboard, "wl_keyboard");

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

    pub fn release(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::RELEASE_OP);
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.release()",
            self.interface, self.id
        );
        conn.write_request(msg.data());
    }
    pub fn parse_event(&self, event: Event<'_>) -> WlKeyboardEvent<'_> {
        let parser = event.parser();
        match event.header.opcode {
            Self::KEYMAP_OP => {
                let format = parser.get_u32();
                let size = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.keymap(format: {}, size: {})",
                    self.interface, self.id, format, size,
                );
                WlKeyboardEvent::Keymap { format, size }
            }
            Self::ENTER_OP => {
                let serial = parser.get_u32();
                let surface = parser.get_u32();
                let keys = parser.get_array_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.enter(serial: {}, surface: {}, keys: {:?})",
                    self.interface, self.id, serial, surface, keys
                );
                WlKeyboardEvent::Enter {
                    serial,
                    surface,
                    keys,
                }
            }
            Self::LEAVE_OP => {
                let serial = parser.get_u32();
                let surface = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.leave(serial: {}, surface: {})",
                    self.interface, self.id, serial, surface
                );
                WlKeyboardEvent::Leave { serial, surface }
            }
            Self::KEY_OP => {
                let serial = parser.get_u32();
                let time = parser.get_u32();
                let key = parser.get_u32();
                let state = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.key(serial: {}, time: {}, key: {}, state: {})",
                    self.interface, self.id, serial, time, key, state
                );
                WlKeyboardEvent::Key {
                    serial,
                    time,
                    key,
                    state,
                }
            }
            Self::MODIFIERS_OP => {
                let serial = parser.get_u32();
                let mods_depressed = parser.get_u32();
                let mods_latched = parser.get_u32();
                let mods_locked = parser.get_u32();
                let group = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.modifiers(serial: {}, mods_depressed: {}, mods_latched: {}, mods_locked: {})",
                    self.interface, self.id, serial, mods_depressed, mods_latched, mods_locked
                );
                WlKeyboardEvent::Modifiers {
                    serial,
                    mods_depressed,
                    mods_latched,
                    mods_locked,
                    group,
                }
            }
            Self::REPEAT_INFO_OP => {
                let rate = parser.get_u32();
                let delay = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: ==> {}#{}.repeat_info(rate: {}, delay: {})",
                    self.interface, self.id, rate, delay
                );
                WlKeyboardEvent::RepeatInfo { rate, delay }
            }
            _ => unreachable!(),
        }
    }
}

impl_obj_prox!(WlTouch, "wl_touch");

// TODO
impl WlTouch {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            interface: "wl_touch",
        }
    }
}

impl_obj_prox!(WlCompositor, "wl_compositor");

impl WlCompositor {
    pub const CREATE_SURFACE_OP: u16 = 0;
    pub const CREATE_REGION_OP: u16 = 1;

    pub fn create_surface(&self, conn: &Connection) -> WlSurface {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::CREATE_SURFACE_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.create_surface(new_id: {})",
            self.interface, self.id, id,
        );
        WlSurface::from_id(id)
    }

    pub fn create_region(&self, conn: &Connection) -> WlRegion {
        let id = conn.new_id();
        let mut msg = Message::<12>::new(self.id, Self::CREATE_REGION_OP);
        msg.write_u32(id).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.create_region(new_id: {})",
            self.interface, self.id, id,
        );
        WlRegion::from_id(id)
    }
}

impl_obj_prox!(WlSurface, "wl_surface");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WlSurfaceEvent {
    Enter { output: u32 },
    Leave { output: u32 },
    PrefferedBufferScale { factor: i32 },
    PrefferedBufferTransform { transform: WlOutputTransform },
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

    pub(crate) const ENTER_OP: u16 = 0;
    pub(crate) const LEAVE_OP: u16 = 1;
    pub(crate) const PREFERED_BUFFER_SCALE_OP: u16 = 2;
    pub(crate) const PREFERRED_BUFFER_TRANSFORM_OP: u16 = 3;

    pub fn parse_event(&self, event: Event<'_>) -> WlSurfaceEvent {
        let mut parser = event.parser();
        match event.header.opcode {
            Self::ENTER_OP => {
                let output = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: {}#{}.enter(output: {})",
                    self.interface, self.id, output,
                );
                WlSurfaceEvent::Enter { output }
            }
            Self::LEAVE_OP => {
                let output = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: {}#{}.leave(output: {})",
                    self.interface, self.id, output,
                );
                WlSurfaceEvent::Leave { output }
            }
            Self::PREFERRED_BUFFER_TRANSFORM_OP => {
                let transform = parser.get_u32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: {}#{}.preffered_buffer_transform(transform: {})",
                    self.interface, self.id, transform,
                );
                WlSurfaceEvent::PrefferedBufferTransform {
                    transform: WlOutputTransform::from(transform),
                }
            }
            Self::PREFERED_BUFFER_SCALE_OP => {
                let factor = parser.get_i32();
                eprintln!(
                    "[\x1b[32mDEBUG\x1b[0m]: {}#{}.preffered_buffer_scale(scale: {})",
                    self.interface, self.id, factor,
                );
                WlSurfaceEvent::PrefferedBufferScale { factor }
            }
            _ => unreachable!(),
        }
    }
    pub fn destroy(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
            self.interface, self.id
        );
    }

    pub fn attach(&self, conn: &Connection, wl_buffer: Option<&WlBuffer>, x: i32, y: i32) {
        let mut msg = Message::<20>::new(self.id, Self::ATTACH_OP);
        let buf_id = wl_buffer.map_or(0, |buf| buf.id);
        msg.write_u32(buf_id).write_i32(x).write_i32(y).build();
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.attach(wl_buffer: {}, x: {}, y: {})",
            self.interface, self.id, buf_id, x, y
        );
        conn.write_request(msg.data());
    }

    pub fn damage(&self, conn: &Connection, x: i32, y: i32, w: i32, h: i32) {
        let mut msg = Message::<24>::new(self.id, Self::DAMANGE_OP);
        msg.write_i32(x)
            .write_i32(y)
            .write_i32(w)
            .write_i32(h)
            .build();
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.damage(x: {}, y: {}, w: {}, h: {})",
            self.interface, self.id, x, y, w, h
        );
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
        let reg_id = wl_region.map_or(0, |w| w.id);
        msg.write_u32(reg_id).build();
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_opaque_region(wl_region: {})",
            self.interface, self.id, reg_id
        );
        conn.write_request(msg.data());
    }

    pub fn set_input_region(&self, conn: &Connection, wl_region: Option<WlRegion>) {
        let mut msg = Message::<12>::new(self.id, Self::SET_INPUT_REGION_OP);
        let reg_id = wl_region.map_or(0, |w| w.id);
        msg.write_u32(reg_id).build();
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_input_region(wl_region: {})",
            self.interface, self.id, reg_id
        );
        conn.write_request(msg.data());
    }

    pub fn commit(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::COMMIT_OP);
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.commit()",
            self.interface, self.id,
        );
        conn.write_request(msg.data());
    }

    pub fn set_buffer_transform(&self, conn: &Connection, transform: WlOutputTransform) {
        let mut msg = Message::<12>::new(self.id, Self::SET_BUFFER_TRANSFORM_OP);
        let transform = transform as i32;
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_buffer_transform(transform: {})",
            self.interface, self.id, transform
        );
        msg.write_i32(transform).build();
        conn.write_request(msg.data());
    }

    pub fn set_buffer_scale(&self, conn: &Connection, scale: i32) {
        let mut msg = Message::<12>::new(self.id, Self::SET_BUFFER_SCALE_OP);
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_buffer_scale(scale: {})",
            self.interface, self.id, scale
        );
        msg.write_i32(scale).build();
        conn.write_request(msg.data());
    }

    pub fn damage_buffer(&self, conn: &Connection, x: i32, y: i32, w: i32, h: i32) {
        let mut msg = Message::<24>::new(self.id, Self::DAMANGE_BUFFER_OP);
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.damage_buffer(x: {}, y: {}, w: {}, h: {})",
            self.interface, self.id, x, y, w, h
        );
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
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.offset(x: {}, y: {})",
            self.interface, self.id, x, y
        );
        conn.write_request(msg.data());
    }
}

impl_obj_prox!(WlRegion, "wl_region");

impl WlRegion {}

impl_obj_prox!(WlBuffer, "wl_buffer");

#[derive(Debug)]
pub enum WlBufferEvent {
    Release,
}

impl WlBuffer {
    pub(crate) const DESTROY_OP: u16 = 0;

    pub(crate) const RELEASE_OP: u16 = 0;
    pub fn parse_event(&self, event: Event<'_>) -> WlBufferEvent {
        if event.header.opcode == Self::RELEASE_OP {
            return WlBufferEvent::Release;
        }
        unreachable!()
    }

    pub fn destroy(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg.data());
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl From<u32> for WlOutputTransform {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::D90,
            2 => Self::D180,
            3 => Self::D270,
            4 => Self::Flipped,
            5 => Self::Flipped90,
            6 => Self::Flipped180,
            7 => Self::Flipped270,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WlOutputMode {
    Current = 1,
    Preffered = 2,
}

impl From<i32> for WlOutputMode {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Current,
            2 => Self::Preffered,
            _ => unreachable!(),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
            _ => unreachable!(),
        }
    }
}

impl_obj_prox!(WlOutput, "wl_output");

impl WlOutput {
    pub(crate) const RELEASE_OP: u16 = 0;

    pub(crate) const GEOMETRY_OP: u16 = 0;
    pub(crate) const MODE_OP: u16 = 1;
    pub(crate) const DONE_OP: u16 = 2;
    pub(crate) const SCALE_OP: u16 = 3;
    pub(crate) const NAME_OP: u16 = 4;
    pub(crate) const DESCRIPTION_OP: u16 = 5;

    pub fn release(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::RELEASE_OP);
        conn.write_request(msg.data());
    }

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
                transform: WlOutputTransform::from(parser.get_u32()),
            },
            Self::MODE_OP => WlOutputEvent::Mode {
                flags: WlOutputMode::from(parser.get_i32()),
                width: parser.get_i32(),
                height: parser.get_i32(),
                refresh: parser.get_i32(),
            },
            Self::DONE_OP => WlOutputEvent::Done,
            Self::SCALE_OP => WlOutputEvent::Scale {
                factor: parser.get_i32(),
            },
            Self::NAME_OP => WlOutputEvent::Name {
                name: parser.get_string(),
            },
            Self::DESCRIPTION_OP => WlOutputEvent::Description {
                description: parser.get_string(),
            },
            _ => unreachable!(),
        }
    }
}

impl_obj_prox!(WlShm, "wl_shm");

impl WlShm {
    pub(crate) const CREATE_POOL: u16 = 0;

    pub fn create_pool(&self, conn: &Connection, fd: i32, size: i32) -> WlShmPool {
        conn.flush();

        let id = conn.new_id();
        let mut msg = Message::<20>::new(self.id, 0);
        msg.write_u32(id).write_i32(size).build();

        unsafe {
            let mut buf = [0i8; unsafe { libc::CMSG_SPACE(size_of::<i32>() as u32) as usize }];
            let mut io = libc::iovec {
                iov_base: msg.data_mut().as_mut_ptr().cast(),
                iov_len: msg.data().len(),
            };
            let mut msghdr = libc::msghdr {
                msg_iov: &mut io as *mut _,
                msg_iovlen: 1,
                msg_control: &mut buf[..] as *mut [i8] as *mut _,
                msg_controllen: buf.len(),
                msg_name: core::ptr::null_mut(),
                msg_namelen: 0,
                msg_flags: 0,
            };

            let mut cmsghdr = libc::CMSG_FIRSTHDR(&msghdr as *const _);

            (*cmsghdr).cmsg_len = libc::CMSG_LEN(4) as usize;
            (*cmsghdr).cmsg_level = libc::SOL_SOCKET;
            (*cmsghdr).cmsg_type = libc::SCM_RIGHTS;

            *(libc::CMSG_DATA(cmsghdr) as *mut libc::c_int) = fd;
            msghdr.msg_controllen = libc::CMSG_SPACE(size_of::<i32>() as u32) as usize;

            if libc::sendmsg(conn.display_fd(), &msghdr as *const libc::msghdr, 0) == -1 {
                panic!("Failed to sendmsg, {}", std::io::Error::last_os_error());
            }
        }

        eprintln!(
            "\x1b[32m\x1b[32m[DEBUG]\x1b[0m\x1b[0m: wl_shm#{}.create_pool(wl_shm_pool#{}, fd: {}, size: {})",
            self.id, id, fd, size
        );
        Object::from_id(id)
    }
}

impl_obj_prox!(WlShmPool, "wl_shm_pool");

impl WlShmPool {
    pub(crate) const CREATE_BUFFER: u16 = 0;

    pub fn create_buffer(
        &self, conn: &Connection, offset: i32, w: i32, h: i32, stride: i32, format: u32,
    ) -> WlBuffer {
        let id = conn.new_id();
        let mut msg = Message::<50>::new(self.id, Self::CREATE_BUFFER);
        msg.write_u32(id)
            .write_i32(offset)
            .write_i32(w)
            .write_i32(h)
            .write_i32(stride)
            .write_u32(format)
            .build();
        conn.write_request(msg.data());
        Object::from_id(id)
    }
}
