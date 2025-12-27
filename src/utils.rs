#![allow(unused)]

use std::mem::MaybeUninit;

// Pretty much a copy-cat of rust's Vec
#[derive(Debug)]
pub struct Bucket<T, const S: usize> {
    data: MaybeUninit<[T; S]>,
    len:  usize,
}

impl<T, const S: usize> Bucket<T, S> {
    pub fn new() -> Self {
        let data = MaybeUninit::uninit();
        Self {
            data,
            len: 0,
        }
    }

    pub fn full() -> Self {
        let mut me = Self::new();
        me.len = S;
        me
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        let pos = self.len;
        &self.data()[..pos]
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let pos = self.len; // ?? thanks rust
        &mut self.data_mut()[..pos]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        S
    }

    #[inline]
    pub fn clear(&mut self) {
        // see `Vec`.clear
        let elems = self.as_slice_mut();
        unsafe {
            core::ptr::drop_in_place(elems);
        }
        self.len = 0;
    }

    #[inline]
    pub fn can_fit(&self, len: usize) -> bool {
        self.capacity() - self.len >= len
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
    fn data(&self) -> &[T; S] {
        unsafe { self.data.as_ptr().as_ref().unwrap_unchecked() }
    }

    #[inline]
    fn data_mut(&mut self) -> &mut [T; S] {
        unsafe { self.data.as_mut_ptr().as_mut().unwrap_unchecked() }
    }

    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Copy,
    {
        if (!self.can_fit(slice.len())) {
            panic!("Provided slice is larger than the available space");
        }
        let begin = self.len;
        let end = slice.len() + begin;
        self.data_mut()[begin..end].copy_from_slice(slice);
        self.len += slice.len();
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr().cast()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr().cast()
    }

    #[inline]
    pub fn fill(&mut self, item: T)
    where
        T: Copy,
    {
        self.data_mut().fill(item)
    }
}

// impl<T, const S: usize> Drop for Bucket<T,S> {
//     fn drop(&mut self) {
//         self.clear();
//     }
// }

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
        const SIZE: usize = 100;
        let mut bucket = Bucket::<u8, SIZE>::new();
        bucket.extend_from_slice(&[0; 45]);
        assert_eq!(bucket.len(), 45);
    }

    #[test]
    fn bucket_pop() {
        const SIZE: usize = 45;
        let mut bucket = Bucket::<u8, SIZE>::new();
        bucket.extend_from_slice(&[0; 10]);
        while let Some(_) = bucket.pop() {}
        assert_eq!(bucket.len(), 0);
    }

    #[test]
    fn bucket_clear() {
        const SIZE: usize = 45;
        let mut bucket = Bucket::<u8, SIZE>::new();
        bucket.extend_from_slice(&[0; 10]);
        assert_eq!(bucket.len(), 10);
        bucket.clear();
        assert_eq!(bucket.len(), 0);
    }

    #[test]
    fn bucket_push() {
        const SIZE: usize = 45;
        let mut bucket = Bucket::<u8, SIZE>::new();
        bucket.push(134);
        bucket.push(80);
        bucket.push(3);
        assert_eq!(bucket.as_slice()[0], 134);
        assert_eq!(bucket.as_slice()[1], 80);
        assert_eq!(bucket.as_slice()[2], 3);
        let i = bucket.pop();
        assert_eq!(i, Some(3));
    }

    #[test]
    fn bucket_crashy() {
        const SIZE: usize = 45;
        #[derive(Debug)]
        struct Droppy(u32);
        impl Drop for Droppy {
            fn drop(&mut self) {
                // println!("Dropped droppy: {:?}", self);
            }
        }
        let mut bucket = Bucket::<Droppy, SIZE>::new();
        bucket.push(Droppy(10));
        bucket.push(Droppy(5));
        bucket.clear(); // Should not double free
        let mut bucket = Bucket::<String, SIZE>::new();
        bucket.push("fsd".into());
        bucket.push("fsd".into());
        bucket.clear(); // Should not double free
    }
}
