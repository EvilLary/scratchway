use std::cell::Cell;

use crate::log;

#[derive(Debug, Clone, Copy)]
pub struct EventIter<'a> {
    buf: &'a [u8],
}

impl<'a> EventIter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }
}

impl<'a> Iterator for EventIter<'a> {
    type Item = WlEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < Header::HEADER_SIZE {
            return None;
        }
        let header = &self.buf[0..Header::HEADER_SIZE];
        let header = Header::from_slice(header);

        // FIXME: Implement some mechanism to keep old data there
        if self.buf.len() < header.size as usize {
            eprintln!(
                "[\x1b[32mERROR\x1b[0m]: Recieived buffer is less than advertised size in the header: {:?},
                discarding the entire buffer",
                header
            );
            return None;
        }

        let Some(data) = self.buf.get(Header::HEADER_SIZE..header.size as usize) else {
            log!(
                ERR,
                "Malformed event with header: {:?}, discarding the entire buffer {:?}",
                header,
                self.buf
            );
            return None; // Thanks kwin
        };

        if self.buf.len() <= header.size as usize {
            self.buf = &[];
        } else {
            self.buf = &self.buf[header.size as usize..];
        }

        Some(WlEvent { header, data })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WlEvent<'a> {
    pub header: Header,
    pub data: &'a [u8],
}

impl<'a> WlEvent<'a> {
    pub fn parser(&self) -> EventDataParser<'a> {
        EventDataParser::new(self.data)
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
    pub fn new(id: u32, opcode: u16, size: u16) -> Self {
        Self { id, opcode, size }
    }
    pub fn from_slice(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), std::mem::size_of::<Self>());
        // Safety: We've already asserted slice length
        // The safe ugly way is not different from using transmute
        unsafe {
            core::mem::transmute_copy::<[u8; Self::HEADER_SIZE], Self>(
                &slice.try_into().unwrap_unchecked(),
            )
        }
    }
}

// #[derive(Debug, Clone, Copy)]
pub struct EventDataParser<'a> {
    pub data: &'a [u8],
    idx: Cell<usize>,
}

impl<'a> EventDataParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            idx: Cell::new(0),
        }
    }

    pub fn get_u16(&self) -> u16 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = u16::from_ne_bytes([data[0], data[1]]);
        self.idx.replace(idx + core::mem::size_of::<u16>());
        num
    }

    pub fn get_fixed(&self) -> f32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]) as f32;
        self.idx.replace(idx + core::mem::size_of::<i32>());
        num / 256.0
    }

    pub fn get_u32(&self) -> u32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        // let num = u32::from_ne_bytes(data[0..4]);
        let num = u32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        self.idx.replace(idx + core::mem::size_of::<u32>());
        num
    }

    pub fn get_string<'b>(&'a self) -> &'b str {
        let str_len = self.get_u32() as usize;
        if str_len == 0 { // Test this
            return "";
        }
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let padded_len = roundup(str_len, 4);
        // Null terminator not included
        let str = &data[..str_len - 1];
        self.idx.replace(idx + padded_len);

        // FIXME: This should be removed once migrating to a dipatcher model is done
        // SAFETY: the reference behind message is valid for as long
        // as the event.data is valid, Rust just can't know it
        unsafe {
            let len = str.len();
            let ptr = str.as_ptr();
            let slice = core::slice::from_raw_parts(ptr, len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    pub fn get_array<'b>(&'a self) -> &'b [u32] {
        let array_len = self.get_u32() as usize;
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let array = unsafe {
            let ptr = data[..array_len].as_ptr().cast();
            core::slice::from_raw_parts(ptr, array_len / size_of::<u32>())
        };
        self.idx.replace(idx + array_len);
        array
    }

    pub fn get_i32(&self) -> i32 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        self.idx.replace(idx + core::mem::size_of::<u32>());
        num
    }
}

fn roundup(value: usize, mul: usize) -> usize {
    (((value - 1) / mul) + 1) * mul
}

#[derive(Debug, Clone)]
pub struct Message<const S: usize> {
    buf: [u8; S],
    len: usize,
}

impl<const S: usize> Message<S> {
    pub fn new(id: u32, op: u16) -> Self {
        let mut msg = Message::empty();
        msg.write_u32(id).write_u16(op).write_u16(8);
        msg
    }

    pub fn build(&mut self) {
        self.buf[6..8].copy_from_slice(&(self.len as u16).to_ne_bytes());
    }

    fn empty() -> Self {
        Self {
            buf: [0; S],
            len: 0,
        }
    }

    pub fn write_i32(&mut self, value: i32) -> &mut Self {
        const SIZE: usize = size_of::<i32>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self
    }

    pub fn write_u32(&mut self, value: u32) -> &mut Self {
        const SIZE: usize = size_of::<u32>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self
    }

    // TODO: Does this actually work??
    pub fn write_fixed(&mut self, value: f32) -> &mut Self {
        let wl_fixed = f32::to_bits((value * 256.0).round());
        self.write_u32(wl_fixed);
        self
    }

    pub fn write_u16(&mut self, value: u16) -> &mut Self {
        const SIZE: usize = size_of::<u16>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self
    }

    pub fn write_string(&mut self, str: impl AsRef<str>) -> &mut Self {
        let str = str.as_ref();
        if str.is_empty() { // TODO: test this
            self.write_u32(0);
            return self;
        }
        // null included
        self.write_u32((str.len() + 1) as u32);
        self.buf[self.len..str.len() + self.len].copy_from_slice(str.as_bytes());
        self.len += roundup(str.len() + 1, 4);
        self
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..self.len]
    }

    pub fn data(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
