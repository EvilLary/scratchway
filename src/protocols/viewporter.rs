use crate::connection::Connection;
use crate::protocols::impl_obj_prox;
use crate::protocols::core::*;
use crate::events::*;

pub use crate::protocols::Object;
impl_obj_prox!(WpViewporter, "wp_viewporter");
impl WpViewporter {
    pub(crate) const DESTROY_OP: u16 = 0;
    pub(crate) const GET_VIEW_OP: u16 = 1;

    pub fn destroy(&self, conn: &Connection) {
        let msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
            self.interface, self.id
        );
    }

    /// Extend surface interface for crop and scale
    ///
    /// Instantiate an interface extension for the given wl_surface to crop and scale its content.
    /// If the given wl_surface already has a wp_viewport object associated, the viewport_exists protocol error is raised.
    pub fn get_viewport(
        &self, conn: &Connection, wl_surface: &WlSurface,
    ) -> WpViewport {
        let id = conn.new_id();
        let mut msg = Message::<16>::new(self.id, Self::GET_VIEW_OP);
        msg.write_u32(id).write_u32(wl_surface.id()).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.get_viewport(new_id: {}, wl_surface: {})",
            self.interface, self.id, id, wl_surface.id
        );
        Object::from_id(id)
    }
}

impl_obj_prox!(WpViewport, "wp_viewport");
impl WpViewport {
    pub(crate) const DESTROY_OP: u16 = 0;
    pub(crate) const SET_SOURCE_OP: u16 = 1;
    pub(crate) const SET_DESTINATION_OP: u16 = 2;

    /// Remove scaling and cropping from the surface
    ///
    /// The associated wl_surface's crop and scale state is removed.
    ///  The change is applied on the next wl_surface.commit.
    pub fn destroy(&self, conn: &Connection) {
        let msg = Message::<8>::new(self.id, Self::DESTROY_OP);
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.destroy()",
            self.interface, self.id
        );
    }
    /// Set the source rectangle for cropping
    ///
    /// Set the source rectangle of the associated wl_surface. See wp_viewport for the description, and relation to the wl_buffer size.
    /// If all of x, y, width and height are -1.0, the source rectangle is unset instead.
    /// Any other set of values where width or height are zero or negative, or x or y are negative, raise the bad_value protocol error.
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    pub fn set_source(&self, conn: &Connection, x: f32, y: f32, w: f32, h: f32) {
        let mut msg = Message::<24>::new(self.id, Self::SET_SOURCE_OP);
        msg.write_fixed(x)
            .write_fixed(y)
            .write_fixed(w)
            .write_fixed(h)
            .build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_source(x: {}, y: {}, w: {}, h: {})",
            self.interface, self.id, x, y, w, h
        );
    }
    /// Set the surface size for scaling
    ///
    /// Set the destination size of the associated wl_surface. See wp_viewport for the description, and relation to the wl_buffer size.
    /// If width is -1 and height is -1, the destination size is unset instead.
    /// Any other pair of values for width and height that contains zero or negative values raises the bad_value protocol error.
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    pub fn set_destination(&self, conn: &Connection, w: i32, h: i32) {
        let mut msg = Message::<16>::new(self.id, Self::SET_DESTINATION_OP);
        msg.write_i32(w).write_i32(h).build();
        conn.write_request(msg.data());
        eprintln!(
            "[\x1b[32mDEBUG\x1b[0m]: {}#{}.set_destination(w: {}, h: {})",
            self.interface, self.id, w, h
        );
    }
}
