use std::cell::Cell;

#[derive(Debug)]
pub struct EventIter<'a> {
    buf: &'a [u8],
}

impl<'a> EventIter<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        EventIter { buf: data }
    }
}

impl<'a> Iterator for EventIter<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // let header_size = Header::HEADER_SIZE;
        if self.buf.len() < Header::HEADER_SIZE {
            return None;
        }
        let header = &self.buf[0..Header::HEADER_SIZE];
        let header = Header::from_slice(header);

        if self.buf.len() < header.size as usize {
            return None;
        }

        let data = &self.buf[Header::HEADER_SIZE..header.size as usize];

        if self.buf.len() <= header.size as usize {
            self.buf = &[];
        } else {
            self.buf = &self.buf[header.size as usize..];
        }
        Some(Event { header, data })
    }
}

#[derive(Debug)]
pub struct Event<'a> {
    pub header: Header,
    pub data: &'a [u8],
}

impl<'a> Event<'a> {
    pub fn parser(&self) -> EventDataParser<'a> {
        EventDataParser::new(self.data)
    }
}

#[derive(Debug)]
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
        let id = u32::from_ne_bytes([slice[0], slice[1], slice[2], slice[3]]);
        let opcode = u16::from_ne_bytes([slice[4], slice[5]]);
        let size = u16::from_ne_bytes([slice[6], slice[7]]);
        Self { id, opcode, size }
    }
}

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

    pub fn get_u16(&mut self) -> u16 {
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let num = u16::from_ne_bytes([data[0], data[1]]);
        self.idx.replace(idx + core::mem::size_of::<u16>());
        num
    }

    pub fn get_fixed(&mut self) -> f32 {
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

    pub fn get_string(&'a self) -> &'a str {
        let str_len = self.get_u32() as usize;
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let padded_len = roundup(str_len, 4);
        // Null terminator not included
        let string = &data[..str_len - 1];
        self.idx.replace(idx + padded_len);
        unsafe { core::str::from_utf8_unchecked(string) }
    }

    pub fn get_array_u32(&'a self) -> &'a [u32] {
        let array_len = self.get_u32() as usize;
        let idx = self.idx.get();
        let data = &self.data[idx..];
        let array = unsafe {
            let ptr = data[..array_len].as_ptr() as *const u32;
            core::slice::from_raw_parts(ptr, array_len / size_of::<u32>())
        };
        self.idx.replace(idx + array_len);
        array
    }
}

fn roundup(value: usize, mul: usize) -> usize {
    (((value - 1) / mul) + 1) * mul
}

#[derive(Debug)]
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

    pub fn write_str(&mut self, str: &str) -> &mut Self {
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
