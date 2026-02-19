use crate::{
    connection::{Connection, wire::*},
    log,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: &'static str,
    pub version: u32,
    // TODO
    pub events: &'static [&'static str],
    pub requests: &'static [&'static str],
}


// TODO: use NonZeroU32 so Rust can optimize Option<WAYLAND-INTERFACE>
#[derive(Clone, Copy)]
pub struct WlProxy {
    pub id: u32,
    pub version: u32,
    pub interface: &'static Interface,
}

impl core::fmt::Display for WlProxy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl core::fmt::Debug for WlProxy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}#{}", self.interface.name, self.id))
    }
}

impl core::hash::Hash for WlProxy {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl core::cmp::PartialEq for WlProxy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl core::cmp::Eq for WlProxy {}

impl WlProxy {
    pub(crate) fn new<I: WlInterface>(id: u32, version: u32) -> Self {
        Self {
            id,
            version,
            interface: I::interface(),
        }
    }
}

// type Callback<S> = Box<dyn Fn(&mut S, &mut Connection<S>, WlProxy, WlEvent) + 'static>;
type Callback<S> = fn(&mut S, &mut Connection<S>, WlProxy, WlEvent);

pub(super) struct WlObject<S> {
    pub(super) wl_proxy: WlProxy,
    pub(super) callback: Option<Callback<S>>,
}

impl<S> WlObject<S> {
    fn new<I>(wl_proxy: WlProxy) -> Self
    where
        I: WlInterface,
        S: Listener<I>,
    {
        let callback = Some(Self::gen_event_callback::<I>());
        WlObject { wl_proxy, callback }
    }

    // This gotta be in its own function
    pub(crate) fn gen_event_callback<I>() -> Callback<S>
    where
        I: WlInterface,
        S: Listener<I>,
    {
        |state, conn, proxy, event| {
            let object = I::from_proxy(proxy);
            let event = I::parse_event(&object, conn, &event);
            <S as Listener<I>>::on_event(state, conn, event, object);
        }
    }
}

// fn event_callback<I, S>(state: &mut S, conn: &mut Connection<S>, proxy: WlProxy, event: WlEvent)
// where
//     I: WlInterface,
//     S: Listener<I>,
// {
//     let object = I::from_proxy(proxy);
//     let event = I::parse_event(&object, conn, &event);
//     <S as Listener<I>>::on_event(state, conn, event, object);
// }

impl<S> core::fmt::Debug for WlObject<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> 0x{:x}", self.wl_proxy, unsafe {
            core::mem::transmute::<_, usize>(self.callback)
        })
    }
}

#[derive(Debug)]
pub(super) struct WlObjects<S> {
    pub(super) objects: HashMap<u32, WlObject<S>>,
    pub(super) dead_ids: Vec<u32>,
    pub(super) next_id: u32,
}

impl<S> Default for WlObjects<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> WlObjects<S> {
    pub(super) fn new() -> Self {
        Self {
            objects: HashMap::new(),
            dead_ids: Vec::new(),
            next_id: 1,
        }
    }

    #[inline]
    pub(super) fn create_deaf_wlinterface<I>(&mut self, id: Option<u32>, version: u32) -> I
    where
        I: WlInterface,
    {
        let wl_proxy = WlProxy::new::<I>(id.unwrap_or_else(|| self.new_id()), version);
        self.objects.insert(
            wl_proxy.id,
            WlObject {
                wl_proxy,
                callback: None,
            },
        );
        log!(TRACE, "Created a deaf interface {}", wl_proxy);
        I::from_proxy(wl_proxy)
    }

    pub(super) fn new_id(&mut self) -> u32 {
        self.dead_ids.pop().unwrap_or_else(|| {
            self.next_id += 1;
            self.next_id
        })
    }

    // lol this approach is probably bloating the binary size
    // None id means it's a client object
    pub(super) fn new_wlinterface<I>(&mut self, id: Option<u32>, version: u32) -> I
    where
        I: WlInterface,
        S: Listener<I>,
    {
        let id = id.unwrap_or_else(|| self.new_id());

        let wl_proxy = WlProxy::new::<I>(id, version);
        log!(TRACE, "Created a new interface {}", wl_proxy);

        let wl_object = WlObject::new::<I>(wl_proxy);
        self.objects.insert(wl_proxy.id, wl_object);
        log!(TRACE, "Registered object {}", wl_proxy);

        I::from_proxy(wl_proxy)
    }

    pub(super) fn dispatch_event(&self, state: &mut S, conn: &mut Connection<S>, event: WlEvent) {
        let Some(wl_object) = self.objects.get(&event.header.id) else {
            log!(WARNING, "Got event for untracked object#{}", event.header.id);
            return;
        };
        match wl_object.callback {
            Some(cb) => {
                log!(
                    TRACE,
                    "Dispatching event for {} [op: {}, size: {}]",
                    wl_object.wl_proxy,
                    event.header.opcode,
                    event.header.size
                );
                (cb)(state, conn, wl_object.wl_proxy, event);
            },
            None => {
                log!(TRACE, "Deaf event {:?}", event.header);
            },
        }
    }

    pub(super) fn destroy_object(&mut self, id: u32) {
        log!(TRACE, "Removing object#{}", id);
        if self.objects.remove(&id).is_some() && id < 0xFF000000 {
            self.dead_ids.push(id);
        }
    }
}

// While I don't usually prefer this approach, writing callback manually is tiresome
pub trait Listener<O>: Sized
where
    O: WlInterface,
{
    fn on_event<'a>(&mut self, conn: &mut Connection<Self>, event: O::Event<'a>, object: O);
}

pub trait WlInterface: Sized + core::fmt::Debug {
    type Event<'a>;

    fn parse_event<'a, S>(
        &self, conn: &mut crate::connection::Connection<S>, event: &'a crate::wire::WlEvent,
    ) -> Self::Event<'a>;

    fn version(&self) -> u32;

    fn id(&self) -> u32;

    fn interface() -> &'static Interface;

    fn proxy(&self) -> WlProxy {
        WlProxy {
            id: self.id(),
            version: self.version(),
            interface: Self::interface(),
        }
    }

    fn from_proxy(wl_proxy: WlProxy) -> Self;
}
