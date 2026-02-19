#![allow(unused)]
pub static TRACE: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| unsafe {
    let env = libc::getenv(c"SCR_TRACE".as_ptr()).cast_const();
    !env.is_null() && libc::strcmp(env, c"1".as_ptr().cast()) == 0
});

#[derive(Debug)]
pub struct Connection<S> {
    pub debug: bool,
    socket: UnixStream,

    event_queue: VecDeque<WlEvent>,
    wl_objects: WlObjects<S>,
    wl_registry: WlRegistry,

    reader: WaylandBuffer<Reader>,
    writer: WaylandBuffer<Writer>,
}

impl<S> Connection<S> {
    pub const WL_DISPLAY_ID: u32 = 1;

    pub fn transform<T>(self) -> Connection<T> {
        let Connection {
            debug,
            socket,
            event_queue,
            wl_objects,
            wl_registry,
            reader,
            writer,
        } = self;

        let WlObjects {
            objects,
            dead_ids,
            next_id,
        } = wl_objects;

        let mut new_objects = std::collections::HashMap::new();

        for (id, obj) in objects {
            let WlObject { wl_proxy, callback } = obj;
            new_objects.insert(
                id,
                WlObject {
                    wl_proxy,
                    callback: None,
                },
            );
        }

        let wl_objects = WlObjects {
            next_id,
            dead_ids,
            objects: new_objects,
        };

        Connection {
            debug,
            socket,
            event_queue,
            wl_objects,
            wl_registry,
            reader,
            writer,
        }
    }

    pub fn connect_with_globals() -> io::Result<(Self, Vec<Global>)> {
        struct GlobalList {
            globals: Vec<Global>,
        }

        impl Listener<WlRegistry> for GlobalList {
            fn on_event<'a>(
                &mut self, conn: &mut Connection<Self>, event: <WlRegistry as WlInterface>::Event<'a>,
                object: WlRegistry,
            ) {
                match event {
                    wl_registry::Event::Global {
                        name,
                        interface,
                        version,
                    } => {
                        self.globals.push(Global {
                            name,
                            interface: interface.into(),
                            version,
                        });
                    },
                    _ => {},
                }
            }
        }

        let mut globals = GlobalList { globals: vec![] };

        let mut conn = Connection::<GlobalList>::connect()?;
        conn.undeaf_object((conn.registry()));
        conn.roundtrip(&mut globals)?;

        Ok((conn.transform::<S>(), globals.globals))
    }

    pub fn connect() -> io::Result<Self> {
        let socket_path = {
            let wayland_disp =
                env::var_os("WAYLAND_DISPLAY").ok_or(io::Error::other("WAYLAND_DISPLAY isn't set"))?;
            let runtime_dir =
                env::var_os("XDG_RUNTIME_DIR").ok_or(io::Error::other("XDG_RUNTIME_DIR isn't set"))?;
            PathBuf::from(runtime_dir).join(wayland_disp)
        };

        let socket = UnixStream::connect(&socket_path)?;
        log!(TRACE, "connected to wayland socket at {:?}", socket_path);

        let mut wl_objects = WlObjects::new();
        let wl_display = wl_objects.create_deaf_wlinterface::<WlDisplay>(Some(1), 1);
        let wl_registry = WlRegistry::from_proxy(WlProxy::new::<WlRegistry>(2, 1));

        let debug = unsafe {
            let env = libc::getenv(c"WAYLAND_DEBUG".as_ptr()).cast_const();
            !env.is_null() && libc::strcmp(env, c"1".as_ptr().cast()) == 0
        };

        let mut me = Self {
            reader: WaylandBuffer::<Reader>::new(),
            writer: WaylandBuffer::<Writer>::new(),
            socket,
            event_queue: VecDeque::new(),
            wl_objects,
            wl_registry,
            debug,
        };

        let wl_registry = wl_display.get_registry_deaf(&mut me);

        Ok(me)
    }

    pub fn display(&self) -> WlDisplay {
        WlDisplay::default()
    }

    pub fn registry(&self) -> WlRegistry {
        self.wl_registry
    }

    pub fn display_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }

    pub fn dispatch_events(&mut self, state: &mut S) {
        while let Some(event) = self.event_queue.pop_front() {
            self.send_event(state, event);
        }
    }

    pub fn read_events(&mut self) -> io::Result<()> {
        let len = self.reader.recv(self.display_fd())?;
        let data = &self.reader.data;
        let mut event_iter = EventIter::new(&data[..len]);

        if self.debug {
            for event in event_iter {
                let id = event.header.id;
                let op = event.header.opcode;
                if let Some(obj) = self.wl_objects.objects.get(&event.header.id) {
                    let ev_name = obj.wl_proxy.interface.events[op as usize];
                    if event.header.id == 1 {
                        let event = self.display().parse_event(self.get_mut(), &event);
                        log!(WAYLAND, "==> wl_display.{:?}", event);
                    } else {
                        log!(
                            WAYLAND,
                            "==> {}.{}(len: {})",
                            obj.wl_proxy,
                            ev_name,
                            event.header.size - 8
                        );
                    }
                } else {
                    log!(WAYLAND, "==> UNTRACKED_OBJECT#{}", id);
                }
                self.event_queue.push_back(event);
            }
        } else {
            self.event_queue.extend(event_iter);
        }

        Ok(())
    }

    pub fn blocking_dispatch(&mut self, state: &mut S) -> io::Result<()> {
        // in case there are pending events
        self.dispatch_events(state);
        self.flush()?;
        self.read_events()?;
        self.dispatch_events(state);
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.send(self.display_fd())
    }

    pub fn roundtrip(&mut self, state: &mut S) -> io::Result<()> {
        self.dispatch_events(state);

        let display = self.display();
        let wl_callback = display.sync(self);

        self.flush()?;
        self.read_events()?;

        while let Some(event) = self.event_queue.pop_front() {
            if wl_callback.id() == event.header.id {
                let _ = wl_callback.parse_event(self, &event);
                break;
            }
            self.send_event(state, event);
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    const fn get_mut(&self) -> &mut Self {
        unsafe { (self as *const Self as *mut Self).as_mut().unwrap_unchecked() }
    }

    fn send_event(&mut self, state: &mut S, event: WlEvent) {
        let id = event.header.id;

        if id == Self::WL_DISPLAY_ID {
            self.handle_wldisplay(&event);
        }
        //  else if id == self.wl_registry.id() {
        //     self.handle_wlregistry(&event)
        // }

        self.wl_objects.dispatch_event(state, self.get_mut(), event);
    }

    // fn handle_wlregistry(&mut self, event: &WlEvent) {
    //     let wl_registry = self.wl_registry;
    //     let event = wl_registry.parse_event(self, event);
    //     match event {
    //         wl_registry::Event::Global {
    //             name,
    //             interface,
    //             version,
    //         } => self.wl_globals.push(Global {
    //             name,
    //             interface: interface.into(),
    //             version,
    //         }),
    //         wl_registry::Event::GlobalRemove { name } => {
    //             if let Some(i) = self.wl_globals.iter().position(|g| g.name == name) {
    //                 self.wl_globals.swap_remove(i);
    //             }
    //         },
    //     }
    // }

    fn handle_wldisplay(&mut self, event: &WlEvent) {
        let parser = event.parser();
        let event = self.display().parse_event(self, event);
        match event {
            crate::wayland::wl_display::Event::Error {
                object_id,
                code,
                message,
            } => {
                // FIXME:
                let proxy = self.wl_objects.objects.get(&object_id).expect("??").wl_proxy;
                log!(
                    ERR,
                    "Protocol Error from {}, code: {}, message: {}",
                    proxy,
                    code,
                    message
                );
                std::process::exit(1);
            },
            crate::wayland::wl_display::Event::DeleteId { id } => {
                self.wl_objects.destroy_object(id);
            },
        }
    }

    #[doc(hidden)]
    pub fn write_request(&mut self, msg: &[u8]) {
        if !self.writer.data.can_fit(msg.len()) {
            log!(TRACE, "Buffer can't fit additional {} bytes", msg.len());
            // TODO: retry it blocks or if it's interrupted
            let _ = self.flush();
        }
        self.writer.data.extend_from_slice(msg);
    }

    #[doc(hidden)]
    pub fn add_fd(&mut self, fd: OwnedFd) {
        self.writer.fds.push_back(fd);
    }

    #[doc(hidden)]
    pub fn get_fd(&mut self) -> Option<OwnedFd> {
        self.reader.get_fd()
    }

    #[doc(hidden)]
    pub fn create_new_object<I>(&mut self, id: Option<u32>, version: u32) -> I
    where
        I: WlInterface,
        S: Listener<I>,
    {
        self.wl_objects.new_wlinterface::<I>(id, version)
    }

    #[doc(hidden)]
    pub fn deaf_wlinterface<I>(&mut self, id: Option<u32>, version: u32) -> I
    where
        I: WlInterface,
    {
        self.wl_objects.create_deaf_wlinterface::<I>(id, version)
    }

    #[doc(hidden)]
    pub fn get_object<I: WlInterface>(&self, id: u32) -> I {
        I::from_proxy(self.wl_objects.objects.get(&id).unwrap().wl_proxy)
    }

    /// You must use this to listen for server generated objects,
    /// wl_registry or wl_display
    ///
    /// # Panics
    ///
    /// Panics if the object doesn't exists
    pub fn undeaf_object<I>(&mut self, wl_interface: I)
    where
        I: WlInterface,
        S: Listener<I>,
    {
        let wl_proxy = wl_interface.proxy();
        let wl_object = self
            .wl_objects
            .objects
            .get_mut(&wl_proxy.id)
            .expect("Object doesn't exist");
        log!(TRACE, "Listening to server object {}", wl_proxy);
        wl_object.callback = Some(WlObject::gen_event_callback::<I>());
    }
}

use crate::{
    WlProxy, log,
    objects::{Listener, WlInterface, WlObject, WlObjects},
    wayland::{WlDisplay, WlRegistry, wl_registry},
};
use std::{
    collections::VecDeque,
    env, io,
    ops::RangeBounds,
    os::{
        fd::{AsRawFd, OwnedFd, RawFd},
        unix::net::UnixStream,
    },
    path::PathBuf,
};
use wayio::*;
use wire::*;

pub use types::*;

mod types;
mod wayio;
pub mod wire;
