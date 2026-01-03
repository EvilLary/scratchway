#![allow(unused)]

use std::mem::MaybeUninit;

// Pretty much a copy-cat of rust's Vec
#[derive(Debug)]
pub struct Bucket<T, const S: usize> {
    data: MaybeUninit<[T; S]>,
    len:  usize,
}

impl<T, const S: usize> Bucket<T, S> {
    pub const fn new() -> Self {
        let data = MaybeUninit::uninit();
        Self {
            data,
            len: 0,
        }
    }

    pub const fn full() -> Self {
        let mut me = Self::new();
        me.len = S;
        me
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        let pos = self.len;
        unsafe { &self.data()[..pos] }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let pos = self.len; // ?? thanks rust
        unsafe { &mut self.data_mut()[..pos] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        S
    }

    pub fn clear(&mut self) {
        // see `Vec`.clear
        let elems = self.as_slice_mut();
        unsafe {
            core::ptr::drop_in_place(elems);
        }
        self.len = 0;
    }

    #[inline]
    pub const fn can_fit(&self, len: usize) -> bool {
        self.capacity() - self.len >= len
    }

    pub const unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn push(&mut self, item: T) {
        debug_assert!(self.len < S);
        unsafe {
            core::ptr::write(self.as_mut_ptr().add(self.len), item);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        unsafe { Some(core::ptr::read(self.as_ptr().add(self.len))) }
    }

    #[inline]
    pub const fn empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    unsafe fn data(&self) -> &[T; S] {
        unsafe { self.data.as_ptr().as_ref().unwrap_unchecked() }
    }

    #[inline]
    unsafe fn data_mut(&mut self) -> &mut [T; S] {
        unsafe { self.data.as_mut_ptr().as_mut().unwrap_unchecked() }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr().cast()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr().cast()
    }

    pub fn extend_from_slice(&mut self, slice: impl AsRef<[T]>)
    where
        T: Copy,
    {
        let slice = slice.as_ref();
        if (!self.can_fit(slice.len())) {
            panic!("Provided slice is larger than the available space");
        }
        let begin = self.len;
        let end = slice.len() + begin;
        unsafe { self.data_mut()[begin..end].copy_from_slice(slice) };
        self.len += slice.len();
    }

    #[inline]
    pub fn fill(&mut self, item: T)
    where
        T: Copy,
    {
        unsafe { self.data_mut().fill(item) }
    }

    pub const unsafe fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len * size_of::<T>())
        }
    }
}

impl<T, const S: usize> Drop for Bucket<T, S> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const S: usize> AsRef<[T]> for Bucket<T, S> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const S: usize> AsMut<[T]> for Bucket<T, S> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}

macro_rules! impl_index {
    ($som:ty, $out:ty) => {
        impl<T, const S: usize> std::ops::Index<$som> for Bucket<T, S> {
            type Output = $out;
            fn index(&self, index: $som) -> &Self::Output {
                &self.as_slice()[index]
            }
        }
        impl<T, const S: usize> std::ops::IndexMut<$som> for Bucket<T, S> {
            fn index_mut(&mut self, index: $som) -> &mut Self::Output {
                &mut self.as_slice_mut()[index]
            }
        }
    };
}

impl_index!(usize, T);
impl_index!(std::ops::RangeTo<usize>, [T]);
impl_index!(std::ops::RangeInclusive<usize>, [T]);
impl_index!(std::ops::RangeFull, [T]);
impl_index!(std::ops::RangeToInclusive<usize>, [T]);

macro_rules! syscall {
    ($fn:expr) => {{
        let ret = $fn;
        if (ret == -1) {
            Err(::std::io::Error::last_os_error())
        } else {
            Ok(ret)
        }
    }};
    () => {};
}
pub(crate) use syscall;

#[macro_export]
macro_rules! log {
    (INFO, $($arg:tt)*) => {{
        eprintln!("[\x1b[32mINFO\x1b[0m]: {}", format_args!($($arg)*));
    }};
    (ERR, $($arg:tt)*) => {{
        eprintln!("[\x1b[31mERROR\x1b[0m]: {}", format_args!($($arg)*));
    }};
    (DEBUG, $($arg:tt)*) => {{
        eprintln!("[\x1b[34mDEBUG\x1b[0m]: {}", format_args!($($arg)*));
    }};
    (TRACE, $($arg:tt)*) => {{
        eprintln!("[\x1b[36mTRACE\x1b[0m]: {}", format_args!($($arg)*));
    }};
    (WARNING, $($arg:tt)*) => {{
        eprintln!("[\x1b[33mWARNING\x1b[0m]: {}", format_args!($($arg)*));
    }};
    (WAYLAND, $($arg:tt)*) => {{
        eprintln!("[\x1b[35mWAYLAND-DEBUG\x1b[0m]: {}", format_args!($($arg)*));
    }};
    () => {};
}

#[cfg(test)]
mod bucket_tests {
    use crate::utils::Bucket;

    #[test]
    fn bucket_size() {
        const SIZE: usize = 100;
        {
            let mut bucket = Bucket::<u8, SIZE>::new();
            bucket.extend_from_slice(&[0; 100]);
            assert!(bucket.can_fit(0)); // duh
            assert!(!bucket.can_fit(10));
            assert!(!bucket.can_fit(2));
            assert!(!bucket.can_fit(1));
        }
        {
            let mut bucket = Bucket::<u8, SIZE>::new();
            bucket.extend_from_slice(&[0; 99]);
            assert!(bucket.can_fit(1));
            assert!(!bucket.can_fit(2));
        }
    }

    #[test]
    fn bucket_len() {
        let mut bucket = Bucket::<u8, 50>::new();
        bucket.extend_from_slice(&[0; 45]);
        assert_eq!(bucket.len(), 45);
    }

    #[test]
    fn bucket_pop() {
        let mut bucket = Bucket::<u8, 20>::new();
        bucket.extend_from_slice(&[0; 10]);
        while let Some(_) = bucket.pop() {}
        assert_eq!(bucket.len(), 0);
    }

    #[test]
    fn bucket_clear() {
        let mut bucket = Bucket::<u8, 40>::new();
        bucket.extend_from_slice(&[0; 10]);
        assert_eq!(bucket.len(), 10);
        bucket.clear();
        assert_eq!(bucket.len(), 0);
    }

    #[test]
    fn bucket_push() {
        let mut bucket = Bucket::<u8, 10>::new();
        bucket.push(134);
        bucket.push(80);
        bucket.push(3);
        assert_eq!(bucket[0], 134);
        assert_eq!(bucket[1], 80);
        assert_eq!(bucket[2], 3);
        assert_eq!(bucket.pop(), Some(3));
    }

    #[test]
    fn bucket_index() {
        const SIZE: usize = 45;
        let mut bucket = Bucket::<u32, SIZE>::new();
        bucket.extend_from_slice([1, 5, 6, 10, 0, 54, 67]);
        assert_eq!(bucket[..5], [1, 5, 6, 10, 0]);
        for n in &mut bucket[..2] {
            *n += 1;
        }
        assert_eq!(bucket.as_slice()[0], 2);
        assert_eq!(bucket.as_slice()[1], 6);
        assert_eq!(bucket[1], 6);
        assert_eq!(bucket[..], [2, 6, 6, 10, 0, 54, 67]);
    }

    #[test]
    fn bucket_crashy() {
        const SIZE: usize = 45;
        #[derive(Debug)]
        struct Droppy(u32);
        impl Drop for Droppy {
            fn drop(&mut self) {
                println!("Dropped droppy: {:?}", self);
            }
        }
        {
            let mut bucket = Bucket::<Droppy, SIZE>::new();
            bucket.push(Droppy(10));
            bucket.push(Droppy(5));
            bucket.clear(); // Should not double free
        }
        {
            let mut bucket = Bucket::<String, SIZE>::new();
            bucket.push("one".into());
            bucket.push("two".into());
            assert_eq!(bucket.pop(), Some("two".into()));
            bucket.clear(); // Should not double free
        }
    }

    #[test]
    fn bucket_bytes() {
        let mut bucket = Bucket::<u32, 10>::new();
        bucket.push(10);
        bucket.push(40);
        unsafe {
            let mut output = [0u8; size_of::<u32>() * 2];
            &output[..4].copy_from_slice(&u32::to_ne_bytes(10));
            &output[4..].copy_from_slice(&u32::to_ne_bytes(40));
            assert_eq!(bucket.as_bytes(), output);
        }
    }
}
