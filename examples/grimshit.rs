#![allow(unused)]
use std::io::Write;

use scratchway::prelude::*;
use scratchway::wayland::*;

use scr_protocols::wlr_screencopy_unstable_v1::{
    zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
    zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, *,
};

fn main() -> std::io::Result<()> {
    let conn = Connection::connect()?;
    let wl_display = conn.display();
    let wl_registry = wl_display.get_registry(conn.writer());

    let mut state = GrimShit {
        cbs: vec![
            (wl_display.id(), GrimShit::on_wldisplay),
            (wl_registry.id(), GrimShit::on_wlregistry),
        ],
        shm_data: ShmData::default(),
        screencopy_mgr: None,
        screencopy_frame: None,
        wl_buffer: None,
        wl_shm: None,
        width: 0,
        height: 0,
        stride: 0,
        format: 0,
        outputs: Vec::new(),
        exit: false,
        wl_registry,
        wl_display,
    };

    conn.roundtrip(&mut state);

    if state.wl_shm.is_none() {
        eprintln!("wl_shm isn't bound??");
        return Ok(());
    }

    if state.outputs.is_empty() {
        eprintln!("There are no outputs");
        return Ok(());
    }

    if state.screencopy_mgr.is_none() {
        eprintln!("Compositor doesn't support wlr_screencopy_unstable_v1");
        return Ok(());
    }

    // Get all info about outputs
    conn.roundtrip(&mut state)?;

    let output = state.outputs.first().unwrap();
    let screencopy_mgr = state.screencopy_mgr.as_ref().unwrap();

    let screencopy_frame = screencopy_mgr.capture_output(conn.writer(), 0, &output.wl_output);
    state.add_cb(screencopy_frame.id(), GrimShit::on_screencopyframe);
    state.screencopy_frame = Some(screencopy_frame);
    // conn.roundtrip(&mut state)?;

    while !state.exit {
        conn.dispatch_events(&mut state)?;
    }

    Ok(())
}

#[derive(Debug)]
struct Output {
    port: String,
    width: i32,
    height: i32,
    mode: u32,
    wl_output: wl_output::WlOutput,
    name: u32,
}

type Callback = fn(&mut GrimShit, &Connection, WlEvent<'_>);
#[derive(Debug)]
struct GrimShit {
    wl_registry: wl_registry::WlRegistry,
    wl_display: wl_display::WlDisplay,
    wl_shm: Option<wl_shm::WlShm>,

    screencopy_mgr: Option<ZwlrScreencopyManagerV1>,
    screencopy_frame: Option<ZwlrScreencopyFrameV1>,

    width: u32,
    height: u32,
    stride: u32,
    format: u32,

    wl_buffer: Option<wl_buffer::WlBuffer>,
    outputs: Vec<Output>,

    cbs: Vec<(u32, Callback)>,

    shm_data: ShmData,
    exit: bool,
}

#[derive(Debug, Default)]
struct ShmData {
    data: *mut u8,
}

impl Drop for ShmData {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                libc::munmap(
                    self.data as *mut libc::c_void,
                    core::mem::size_of_val(self.data.as_mut().unwrap()),
                );
            }
        }
    }
}

impl GrimShit {
    fn on_wldisplay(&mut self, conn: &Connection, event: WlEvent) {}

    fn on_screencopyframe(&mut self, conn: &Connection, event: WlEvent) {
        let frame = self.screencopy_frame.as_ref().expect("fsdfl");
        match frame.parse_event(conn.reader(), event) {
            zwlr_screencopy_frame_v1::Event::Buffer {
                format,
                width,
                height,
                stride,
            } => {
                self.format = format;
                self.height = height;
                self.width = width;
                self.stride = stride;
            }
            zwlr_screencopy_frame_v1::Event::BufferDone => {
                let name = {
                    let time = unsafe { libc::time(std::ptr::null_mut()) };
                    format!("grimshit-{}\0", time)
                };
                let name = name.as_ptr().cast();
                let shm_pool_size = self.stride * self.height;
                let shm_fd = unsafe {
                    let flags = libc::O_RDWR | libc::O_EXCL | libc::O_CREAT;
                    libc::shm_open(name, flags, 0o600)
                };

                if shm_fd == -1 {
                    panic!("Couldn't create shm {}", std::io::Error::last_os_error());
                }

                unsafe {
                    if libc::shm_unlink(name) == -1 {
                        panic!("Couldn't unlink shm {}", std::io::Error::last_os_error());
                    }
                    if libc::ftruncate(shm_fd, shm_pool_size as i64) == -1 {
                        panic!("Couldn't truncate shm {}", std::io::Error::last_os_error());
                    }
                }

                let shm_pool = unsafe {
                    let prot = libc::PROT_READ | libc::PROT_WRITE;
                    libc::mmap(
                        std::ptr::null_mut(),
                        shm_pool_size as usize,
                        prot,
                        libc::MAP_SHARED,
                        shm_fd,
                        0,
                    )
                };

                if shm_pool.is_null() {
                    panic!("Couldn't mmap shm {}", std::io::Error::last_os_error());
                }

                let wl_shm = self.wl_shm.as_ref().expect("fsdjkf");

                let wl_shm_pool = wl_shm.create_pool(conn.writer(), shm_fd, shm_pool_size as i32);

                let wl_buffer = wl_shm_pool.create_buffer(
                    conn.writer(),
                    0,
                    self.width as i32,
                    self.height as i32,
                    self.stride as i32,
                    1,
                );
                wl_shm_pool.destroy(conn.writer());

                // self.callbacks.push((wl_buffer.id(), Self::on_wlbuffer));
                frame.copy(conn.writer(), &wl_buffer);
                self.wl_buffer = Some(wl_buffer);

                self.shm_data.data = shm_pool as *mut u8;
            }
            zwlr_screencopy_frame_v1::Event::Failed => {
                panic!("Failed to copy buffer");
            }
            zwlr_screencopy_frame_v1::Event::Ready { .. } => {
                self.exit = true;

                let path = {
                    let time = unsafe { libc::time(std::ptr::null_mut()) };
                    format!("grimshit-{}.ppm", time)
                };
                if let Err(e) = self.save_to_file(&path) {
                    scratchway::log!(ERR, "Failed to write to file: {}", e);
                    return;
                }
                scratchway::log!(INFO, "Saved image to {}", path);
            }
            _ => {}
        }
    }

    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let height = self.height as usize;
        let width = self.width as usize;
        let stride = self.stride as usize;

        let mut file =
            std::io::BufWriter::with_capacity(height * stride, std::fs::File::create(path)?);

        write!(file, "P6\n{} {}\n255\n", width, height)?;

        let buffer = unsafe {
            core::slice::from_raw_parts_mut(self.shm_data.data as *mut u32, (stride * height) / 4)
        };

        for y in 0..height {
            for x in 0..width {
                let pixel = buffer[y * width + x];
                let color = [
                    ((pixel >> 16) & 0xFF) as u8,
                    ((pixel >> 8) & 0xFF) as u8,
                    ((pixel) & 0xFF) as u8,
                ];
                file.write_all(&color)?;
            }
        }

        file.flush()
    }

    fn on_wloutput(&mut self, conn: &Connection, event: WlEvent) {
        let output = self
            .outputs
            .iter_mut()
            .find(|o| event.header.id == o.wl_output.id())
            .expect("Couldn't get output for recieved output event?");
        match output.wl_output.parse_event(conn.reader(), event) {
            wl_output::Event::Mode {
                flags,
                width,
                height,
                refresh,
            } => {
                output.width = width;
                output.height = height;
                output.mode = flags;
            }
            wl_output::Event::Name { name } => {
                scratchway::log!(INFO, "Found output {}", name);
                output.port = name.into();
            }
            _ => {}
        }
    }

    fn on_wlshm(&mut self, conn: &Connection, event: WlEvent) {
        let wl_shm = self.wl_shm.as_ref().expect("hfdosdf");
        match wl_shm.parse_event(conn.reader(), event) {
            _ => {}
        }
    }

    #[rustfmt::skip]
    fn on_wlregistry(&mut self, conn: &Connection, event: WlEvent) {
        match self.wl_registry.parse_event(conn.reader(), event) {
            wl_registry::Event::Global { name, interface, version } => {
                match interface {
                    wl_output::WlOutput::INTERFACE => {
                        let wl_output: wl_output::WlOutput = self.wl_registry.bind(conn.writer(), name, interface, version);
                        self.add_cb(wl_output.id(), Self::on_wloutput);
                        self.outputs.push(Output {
                            name,
                            height: 0,
                            width: 0,
                            wl_output,
                            port: String::new(),
                            mode: 0
                        });

                    }
                    ZwlrScreencopyManagerV1::INTERFACE => {
                        let screencopy_mgr = self.wl_registry.bind(conn.writer(), name, interface, version);
                        self.screencopy_mgr = Some(screencopy_mgr)
                    }
                    wl_shm::WlShm::INTERFACE => {
                        let wl_shm: wl_shm::WlShm = self.wl_registry.bind(conn.writer(), name, interface, version);
                        self.add_cb(wl_shm.id(), Self::on_wlshm);
                        self.wl_shm = Some(wl_shm)
                    }
                    _ => {}
                }
            },
            wl_registry::Event::GlobalRemove { name } => {

            },
        }
    }

    fn add_cb(&mut self, id: u32, cb: Callback) {
        self.cbs.push((id, cb));
    }
}

impl State for GrimShit {
    fn handle_event(&mut self, conn: &Connection, event: WlEvent<'_>) {
        if let Some((_, cb)) = self.cbs.iter().find(|(id, _)| *id == event.header.id) {
            cb(self, conn, event)
        } else {
            scratchway::log!(
                ERR,
                " Unhandled event for id: {}, opcode: {}",
                event.header.id,
                event.header.opcode
            )
        }
    }
}
