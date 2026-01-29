use core::{mem, ops::Deref, slice};

use alloc::{sync::Arc, vec::Vec};

use bytes::{Buf, Bytes};

// Vtable must enforce this behavior
unsafe impl Send for BytesRef {}
unsafe impl Sync for BytesRef {}

#[derive(Clone)]
pub struct BytesRef {
    pub(crate) ptr: *const u8,
    pub(crate) len: usize,
    pub(crate) data: Arc<Bytes>,
}

impl From<Bytes> for BytesRef {
    #[inline]
    fn from(data: Bytes) -> Self {
        BytesRef {
            ptr: data.as_ptr(),
            len: data.len(),
            data: Arc::new(data),
        }
    }
}

impl From<BytesRef> for Bytes {
    #[inline]
    fn from(value: BytesRef) -> Self {
        if value.len == 0 {
            return Bytes::new();
        }
        if value.ptr == value.data.as_ptr() && value.len == value.data.len() {
            return Arc::unwrap_or_clone(value.data);
        }

        let original_start = value.data.as_ptr();
        let offset = unsafe { value.ptr.offset_from(original_start) } as usize;
        value.data.slice(offset..offset + value.len)
    }
}

impl From<BytesRef> for Vec<u8> {
    #[inline]
    fn from(value: BytesRef) -> Self {
        Bytes::from(value).into()
    }
}

impl BytesRef {
    fn new_empty(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: 0,
            data: self.data.clone(),
        }
    }

    /// Create a new borrowed “view” from this `BytesRef`, pointing to `subset`.
    ///
    /// This does not copy any bytes. It reuses the underlying `Arc<Bytes>` to keep
    /// the allocation alive.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that:
    ///
    /// - `subset` is a true subslice of the bytes currently visible through `self`
    ///   (i.e. its address range is fully contained in `self`).
    pub unsafe fn slice_ref(&self, subset: &[u8]) -> Self {
        Self {
            ptr: subset.as_ptr(),
            len: subset.len(),
            data: self.data.clone(),
        }
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    #[inline]
    /// # Safety
    ///
    /// - The caller must ensure `by <= self.len`.
    unsafe fn inc_start(&mut self, by: usize) {
        // should already be asserted, but debug assert for tests
        debug_assert!(self.len >= by, "internal: inc_start out of bounds");
        self.len -= by;
        self.ptr = self.ptr.add(by);
    }

    pub fn split_to(&mut self, at: usize) -> Self {
        if at == self.len() {
            return mem::replace(self, self.new_empty());
        }

        if at == 0 {
            return self.new_empty();
        }

        assert!(
            at <= self.len(),
            "split_to out of bounds: {:?} <= {:?}",
            at,
            self.len(),
        );

        let mut ret = self.clone();

        unsafe { self.inc_start(at) };

        ret.len = at;
        ret
    }
}

impl Deref for BytesRef {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Buf for BytesRef {
    #[inline]
    fn remaining(&self) -> usize {
        self.len
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        self.as_slice()
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        assert!(
            cnt <= self.len,
            "cannot advance past `remaining`: {:?} <= {:?}",
            cnt,
            self.len,
        );

        unsafe {
            self.inc_start(cnt);
        }
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        self.split_to(len).into()
    }
}
