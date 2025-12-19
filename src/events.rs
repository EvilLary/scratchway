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
}

impl<'a> EventDataParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn get_u16(&mut self) -> u16 {
        let num = u16::from_ne_bytes([self.data[0], self.data[1]]);
        self.data = &self.data[2..];
        num
    }

    pub fn get_fixed(&mut self) -> f32 {
        let num = i32::from_ne_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]) as f32;
        self.data = &self.data[4..];
        num / 256.0
    }

    pub fn get_u32(&mut self) -> u32 {
        let num = u32::from_ne_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]);
        self.data = &self.data[4..];
        num
    }

    pub fn get_string(&mut self) -> String {
        let len = self.get_u32() as usize;
        let padded_len = roundup(len, 4);
        // Null terminator not included
        let string = self.data[..len - 1].to_vec();
        self.data = &self.data[padded_len..];
        unsafe { String::from_utf8_unchecked(string) }
    }

    pub fn get_array_u32(&mut self) -> Vec<u32> {
        let len = self.get_u32() as usize;
        let vec = unsafe {
            let ptr = self.data[..len].as_ptr() as *const u32;
            let slice = core::slice::from_raw_parts(ptr, len / size_of::<u32>());
            slice.into()
        };
        self.data = &self.data[len as usize..];
        vec
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
        msg.write_u32(id);
        msg.write_u16(op);
        msg.write_u16(8);
        msg
    }

    fn update_len(&mut self) {
        self.buf[6..8].copy_from_slice(&(self.len as u16).to_ne_bytes());
    }

    fn empty() -> Self {
        Self {
            buf: [0; S],
            len: 0,
        }
    }

    pub fn write_u32(&mut self, value: u32) {
        const SIZE: usize = size_of::<u32>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self.update_len();
    }

    // TODO: Does this actually work??
    pub fn write_fixed(&mut self, value: f32) {
        let wl_fixed = f32::to_bits((value * 256.0).round());
        self.write_u32(wl_fixed)
    }

    pub fn write_u16(&mut self, value: u16) {
        const SIZE: usize = size_of::<u16>();
        self.buf[self.len..self.len + SIZE].copy_from_slice(&value.to_ne_bytes());
        self.len += SIZE;
        self.update_len();
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..self.len]
    }

    pub fn data(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    pub fn write_str(&mut self, str: &str) {
        // null included
        self.write_u32((str.len() + 1) as u32);
        self.buf[self.len..str.len() + self.len].copy_from_slice(str.as_bytes());
        self.len += roundup(str.len() + 1, 4);
        self.update_len();
    }
}
