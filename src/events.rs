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

    pub fn get_u32(&mut self) -> u32 {
        let num = u32::from_ne_bytes([self.data[0], self.data[1], self.data[2], self.data[3]]);
        self.data = &self.data[4..];
        num
    }

    pub fn get_string(&mut self) -> String {
        let len = self.get_u32() as usize;
        let padded_len = crate::roundup(len, 4);
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
