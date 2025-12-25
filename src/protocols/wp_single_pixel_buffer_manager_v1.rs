use crate::connection::Connection;
use crate::protocols::impl_obj_prox;
use crate::protocols::core::*;
use crate::events::*;

pub use crate::protocols::Object;

impl_obj_prox!(WpSinglePixelBufferMgr, "wp_single_pixel_buffer_manager_v1");

impl WpSinglePixelBufferMgr {
    pub(crate) const DESTROY_OP: u16 = 0;
    pub(crate) const CREATE_BUFFER_OP: u16 = 1;

    pub fn destroy(&self, conn: &Connection) {
        let msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        #[cfg(debug_assertions)]
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
            self.interface, self.id
        );
        conn.write_request(msg.data());
    }

    pub fn create_buffer(
        &self, conn: &Connection, r: u32, g: u32, b: u32, a: u32,
    ) -> WlBuffer {
        let id = conn.new_id();
        let mut msg = Message::<28>::new(self.id, Self::CREATE_BUFFER_OP);
        msg.write_u32(id)
            .write_u32(r)
            .write_u32(g)
            .write_u32(b)
            .write_u32(a)
            .build();
        #[cfg(debug_assertions)]
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.create_buffer(new_id: {}, r: {}, g: {}, b: {}, a: {})",
            self.interface, self.id, id, r, g, b, a
        );
        conn.write_request(msg.data());
        Object::from_id(id)
    }
}
