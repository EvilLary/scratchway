use crate::connection::Connection;
use crate::events::*;
use crate::protocols::core::*;
use crate::protocols::impl_obj_prox;

pub use crate::protocols::Object;

impl_obj_prox!(XdgDecorationManager, "zxdg_decoration_manager_v1");

impl XdgDecorationManager {
    pub(crate) const DESTROY_OP: u16 = 0;
    pub(crate) const GET_TOPLEVEL_DECORATION_OP: u16 = 1;

    pub fn destroy(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        #[cfg(debug_assertions)]
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
            self.interface, self.id
        );
        conn.write_request(msg.data());
    }

    pub fn get_toplevel_decoration(
        &self, conn: &Connection, toplevel: crate::protocols::xdg_shell::XdgToplevel,
    ) -> XdgToplevelDecoration {
        let id = conn.new_id();
        let mut msg = Message::<16>::new(self.id, Self::GET_TOPLEVEL_DECORATION_OP);
        msg.write_u32(id).write_u32(toplevel.id()).build();
        conn.write_request(msg.data());
        #[cfg(debug_assertions)]
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_toplevel_decoration(new_id: {}, toplevel: {})",
            self.interface,
            self.id,
            id,
            toplevel.id()
        );
        Object::from_id(id)
    }
}

impl_obj_prox!(XdgToplevelDecoration, "zxdg_toplevel_decoration_v1");

impl XdgToplevelDecoration {
    pub(crate) const DESTROY_OP: u16 = 0;
    pub(crate) const SET_OP: u16 = 1;
    pub(crate) const UNSET_OP: u16 = 2;

    pub(crate) const CONFIGURE_OP: u16 = 2;

    pub fn destroy(&self, conn: &Connection) {
        let mut msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        #[cfg(debug_assertions)]
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
            self.interface, self.id
        );
        conn.write_request(msg.data());
    }
}
