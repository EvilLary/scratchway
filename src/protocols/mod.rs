#![allow(unused)]

pub mod wp_single_pixel_buffer_manager_v1;
pub mod xdg_shell;
pub mod viewporter;
pub mod wlr_layer_shell_unstable_v1;
pub mod core;

pub trait Object {
    type Event;

    fn from_id(id: u32) -> Self;

    #[inline(always)]
    fn id(&self) -> u32;

    fn interface(&self) -> &'static str;

    fn parse_event_obj<T>(&self, event: crate::events::Event<'_>) -> T {
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
            // fn parse_event_obj<T>(&self, event: Event<'_>) -> T {
            //     todo!()
            // }
        }
        impl PartialEq<$prox> for u32 {
            fn eq(&self, other: &$prox) -> bool {
                *self == other.id()
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
