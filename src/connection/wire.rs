use crate::log;
use crate::utils::Bucket;
use std::cell::Cell;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EventIter<'a> {
    buf: &'a [u8],
}

impl<'a> EventIter<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}

impl<'a> Iterator for EventIter<'a> {
    type Item = WlEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < Header::HEADER_SIZE {
            return None;
        }
        let header = &self.buf[0..Header::HEADER_SIZE];
        let header = Header::from_slice(header);

        if self.buf.len() < header.size as usize {
            log!(
                ERR,
                "Recieived buffer is less than advertised size in the header: {:?},
                discarding the entire buffer",
                header
            );
            return None;
        }

        // TODO: find a way to recover from this?
        let Some(data) = self.buf.get(Header::HEADER_SIZE..header.size as usize) else {
            log!(
                ERR,
                "Malformed event with header: {:?}, discarding the entire buffer {:?}",
                header,
                self.buf
            );
            return None; // Thanks kwin
        };

        let data = data.into();

        if self.buf.len() <= header.size as usize {
            self.buf = &[];
        } else {
            self.buf = &self.buf[header.size as usize..];
        }

        Some(WlEvent { header, data })
    }
}

#[derive(Debug)]
pub struct WlEvent {
    pub header: Header,
    pub data: Box<[u8]>,
}

impl WlEvent {
    #[doc(hidden)]
    pub fn parser(&self) -> EventDataParser<'_> {
        EventDataParser::new(self.data.as_ref())
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Header {
    pub id: u32,
    pub opcode: u16,
    pub size: u16,
}

impl Header {
    pub const HEADER_SIZE: usize = size_of::<Self>();

    fn from_slice(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), std::mem::size_of::<Self>());
        // Safety: We've already asserted slice length
        // The safe ugly way is not different from using transmute
        unsafe {
            core::mem::transmute_copy::<[u8; Self::HEADER_SIZE], Self>(&slice.try_into().unwrap_unchecked())
        }
    }
}

#[doc(hidden)]
pub struct EventDataParser<'a> {
    data: &'a [u8],
    idx: Cell<usize>,
}

impl<'a> EventDataParser<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            idx: Cell::new(0),
        }
    }

    pub fn get_u16(&self) -> u16 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = u16::from_ne_bytes([data[0], data[1]]);
        self.idx.replace(idx + size_of::<u16>());
        num
    }

    pub fn get_fixed(&self) -> f32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]) as f32;
        self.idx.replace(idx + size_of::<i32>());
        num / 256.0
    }

    pub fn get_u32(&self) -> u32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        self.idx.replace(idx + size_of::<u32>());
        num
    }

    pub fn get_string<'b>(&'a self) -> &'b str {
        let str_len = self.get_u32() as usize;
        // FIXME
        if str_len == 0 {
            return "";
        }
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let padded_len = str_len.next_multiple_of(4);
        // Null terminator not included
        let str = &data[..str_len - 1];
        self.idx.replace(idx + padded_len);

        // SAFETY: the reference behind message is valid for as long
        // as the event.data is valid, Rust just can't know it
        unsafe {
            let len = str.len();
            let ptr = str.as_ptr();
            let slice = core::slice::from_raw_parts(ptr, len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    pub fn get_array<'b>(&'a self) -> &'b [u8] {
        // This is wrong lol
        let array_len = self.get_u32() as usize;
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let array = unsafe {
            let ptr = data[..array_len].as_ptr().cast();
            core::slice::from_raw_parts(ptr, array_len)
        };
        self.idx.replace(idx + array_len);
        array
    }

    pub fn get_i32(&self) -> i32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        self.idx.replace(idx + size_of::<u32>());
        num
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub struct Message<const S: usize> {
    buf: Bucket<u8, S>,
}

#[doc(hidden)]
impl<const S: usize> Message<S> {
    pub fn new(id: u32, op: u16) -> Self {
        let mut msg = Message::empty();
        msg.write_u32(id);
        msg.write_u16(op);
        msg.write_u16(8);
        msg
    }

    fn empty() -> Self {
        Self { buf: Bucket::new() }
    }

    pub fn build(&mut self) {
        debug_assert!(self.buf.len().is_multiple_of(4));
        let len = self.buf.len() as u16;
        self.buf[6..8].copy_from_slice(&len.to_ne_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.buf.extend_from_slice(value.to_ne_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.buf.extend_from_slice(value.to_ne_bytes());
    }

    // TODO: Does this actually work??
    pub fn write_fixed(&mut self, value: f32) {
        let wl_fixed = f32::to_bits((value * 256.0).round());
        self.write_u32(wl_fixed);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.buf.extend_from_slice(value.to_ne_bytes());
    }

    pub fn write_string(&mut self, str: impl AsRef<str>) {
        let str = str.as_ref();
        if str.is_empty() {
            // TODO: test this
            self.write_u32(0);
            return;
        }
        let cstr_len = str.len() + 1;
        self.write_u32(cstr_len as u32);
        self.buf.extend_from_slice(str);
        self.buf.extend_from_slice([0]);
        self.buf.align_to(4, 0u8);
    }

    pub fn data(&self) -> &[u8] {
        self.buf.as_ref()
    }
}
