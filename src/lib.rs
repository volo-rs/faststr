#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

use bytes::{Bytes, BytesMut};
use simdutf8::basic::{from_utf8, Utf8Error};
use std::{
    borrow::{Borrow, Cow},
    cmp::Ordering,
    convert::Infallible,
    fmt, hash,
    hint::unreachable_unchecked,
    iter,
    mem::MaybeUninit,
    ops::Deref,
    str::FromStr,
    sync::Arc,
};

/// `FastStr` is a string type that try to avoid the cost of clone.
#[derive(Clone)]
pub struct FastStr(Repr);

#[cfg(all(test, target_pointer_width = "64"))]
mod size_asserts {
    static_assertions::assert_eq_size!(super::FastStr, [u8; 40]); // 40 bytes
}

impl FastStr {
    /// Create a new `FastStr` from any type `T` that can be converted to a string slice
    /// (e.g., `String`, `&str`, `Arc<String>`, `Arc<str>`).
    ///
    /// For small strings (up to 38 bytes), this avoids heap allocation, and copies on stack.
    #[inline]
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(Repr::new(text))
    }

    /// Create a new inline `FastStr` (up to 30 bytes long) from a string slice `s`.
    ///
    /// This constructor panics if the length of `s` is greater than 30.
    ///
    /// Note: the inline length is not guaranteed.
    #[inline]
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.13",
        note = "The inline threshold is not stable. Please use `FastStr::new()` instead."
    )]
    pub fn new_inline(s: &str) -> Self {
        Self(Repr::new_inline(s))
    }

    /// Create a new `FastStr` from a byte slice `v`, returning a
    /// `Result<FastStr, Utf8Error>` if the bytes are not valid UTF-8.
    #[inline]
    pub fn new_u8_slice(v: &[u8]) -> Result<Self, Utf8Error> {
        let s = from_utf8(v)?;
        Ok(Self::new(s))
    }

    /// Create a new `FastStr` from a byte slice `v`. This is an unsafe method because
    /// the caller must ensure that the bytes passed to it are valid UTF-8.
    ///
    /// # Safety
    ///
    /// `v` must be valid UTF-8.
    #[inline]
    pub unsafe fn new_u8_slice_unchecked(v: &[u8]) -> Self {
        let s = unsafe { std::str::from_utf8_unchecked(v) };
        Self::new(s)
    }

    /// Create an empty `FastStr`.
    #[inline]
    pub const fn empty() -> Self {
        Self(Repr::empty())
    }

    /// Create a new `FastStr` from an `Arc<str>`.
    #[inline]
    pub fn from_arc_str(s: Arc<str>) -> Self {
        Self(Repr::from_arc_str(s))
    }

    /// Create a new `FastStr` from a `String`.
    #[inline]
    pub fn from_string(s: String) -> Self {
        Self(Repr::from_string(s))
    }

    /// Create a new `FastStr` from an `Arc<String>`.
    #[inline]
    pub fn from_arc_string(s: Arc<String>) -> Self {
        Self(Repr::from_arc_string(s))
    }

    /// Create a new `FastStr` from a `BytesMut` object, returning a
    /// `Result<FastStr, Utf8Error>` if the bytes are not valid UTF-8.
    #[inline]
    pub fn from_bytes(b: Bytes) -> Result<Self, Utf8Error> {
        from_utf8(&b)?;
        // Safety: we have checked b is utf-8 valid
        Ok(unsafe { Self::from_bytes_unchecked(b) })
    }

    /// Create a new `FastStr` from a `Bytes` object. This is an unsafe method
    /// because the caller must ensure that the bytes passed to it are valid UTF-8.
    ///
    /// # Safety
    ///
    /// `b` must be valid UTF-8.
    #[inline]
    pub unsafe fn from_bytes_unchecked(b: Bytes) -> Self {
        Self(Repr::from_bytes_unchecked(b))
    }

    /// Create a new `FastStr` from a `BytesMut` object, returning a
    /// `Result<FastStr, Utf8Error>` if the bytes are not valid UTF-8.
    #[inline]
    pub fn from_bytes_mut(b: BytesMut) -> Result<Self, Utf8Error> {
        from_utf8(&b)?;
        // Safety: we have checked b is utf-8 valid
        Ok(unsafe { Self::from_bytes_mut_unchecked(b) })
    }

    /// Create a new `FastStr` from a `BytesMut` object. This is an unsafe method
    /// because the caller must ensure that the bytes passed to it are valid UTF-8.
    ///
    /// # Safety
    ///
    /// `b` must be valid UTF-8.
    #[inline]
    pub unsafe fn from_bytes_mut_unchecked(b: BytesMut) -> Self {
        let v = b.freeze();
        Self::from_bytes_unchecked(v)
    }

    /// Create a new `FastStr` from a static string slice.
    #[inline]
    pub const fn from_static_str(s: &'static str) -> Self {
        Self(Repr::StaticStr(s))
    }

    /// Create a new `FastStr` from a `Vec<u8>`, returning a
    /// `Result<FastStr, Utf8Error>` if the bytes are not valid UTF-8.
    #[inline]
    pub fn from_vec_u8(v: Vec<u8>) -> Result<Self, Utf8Error> {
        from_utf8(&v)?;
        // Safety: we have checked b is utf-8 valid
        Ok(unsafe { Self::from_vec_u8_unchecked(v) })
    }

    /// Create a new `FastStr` from a `Vec<u8>`. This is an unsafe method because
    /// the caller must ensure that the bytes passed to it are valid UTF-8.
    ///
    /// # Safety
    ///
    /// `v` must be valid UTF-8.
    #[inline]
    pub unsafe fn from_vec_u8_unchecked(v: Vec<u8>) -> Self {
        Self::from_bytes_unchecked(v.into())
    }

    /// Create a new `FastStr` from a byte slice `v`, returning a
    /// `Result<FastStr, Utf8Error>` if the bytes are not valid UTF-8.
    #[deprecated(
        since = "0.2.13",
        note = "This method is not really zero-cost. Use `new_u8_slice` instead."
    )]
    #[inline]
    pub fn from_u8_slice(v: &[u8]) -> Result<Self, Utf8Error> {
        Self::new_u8_slice(v)
    }

    /// Create a new `FastStr` from a byte slice `v`. This is an unsafe method because
    /// the caller must ensure that the bytes passed to it are valid UTF-8.
    ///
    /// # Safety
    ///
    /// `v` must be valid UTF-8.
    #[deprecated(
        since = "0.2.13",
        note = "This method is not really zero-cost. Use `new_u8_slice_unchecked` instead."
    )]
    #[inline]
    pub unsafe fn from_u8_slice_unchecked(v: &[u8]) -> Self {
        Self::new_u8_slice_unchecked(v)
    }
}

impl FastStr {
    /// Return the `FastStr` as a string slice.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Consumes and converts the `FastStr` into a `Bytes` object.
    #[inline(always)]
    pub fn into_bytes(self) -> Bytes {
        self.0.into_bytes()
    }

    /// Return the `FastStr` length.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return `true` if the `FastStr` is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return a new `FastStr` that represents a subset of the current string.
    #[inline(always)]
    pub fn slice_ref(&self, subset: &str) -> Self {
        Self(self.0.slice_ref(subset.as_bytes()))
    }

    /// Return a new `FastStr` starting at index `start` and ending at index `end`. `[start..end)`
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the string between `start` and `end` is valid utf-8.
    #[inline(always)]
    pub unsafe fn index(&self, start: usize, end: usize) -> Self {
        Self(self.0.slice_ref(&self.as_bytes()[start..end]))
    }

    /// Consumes and converts the `FastStr` into a `String` at best effort.
    #[deprecated(
        since = "0.2.13",
        note = "This method does not really express the `into` semantic. Use `to_string` instead."
    )]
    #[inline(always)]
    pub fn into_string(self) -> String {
        #[allow(deprecated)]
        self.0.into_string()
    }

    fn from_char_iter<I: iter::Iterator<Item = char>>(mut iter: I) -> Self {
        let (min_size, _) = iter.size_hint();
        if min_size > INLINE_CAP {
            let s: String = iter.collect();
            return Self(Repr::Bytes(Bytes::from(s)));
        }
        let mut len = 0;
        let mut buf = [0u8; INLINE_CAP];
        while let Some(ch) = iter.next() {
            let size = ch.len_utf8();
            if size + len > INLINE_CAP {
                let (min_remaining, _) = iter.size_hint();
                let mut s = String::with_capacity(size + len + min_remaining);
                s.push_str(unsafe { core::str::from_utf8_unchecked(&buf[..len]) });
                s.push(ch);
                s.extend(iter);
                return Self(Repr::Bytes(Bytes::from(s)));
            }
            ch.encode_utf8(&mut buf[len..]);
            len += size;
        }
        Self(Repr::Inline { len, buf })
    }
}

impl Default for FastStr {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl AsRef<[u8]> for FastStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for FastStr {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for FastStr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl From<FastStr> for String {
    #[inline]
    fn from(val: FastStr) -> Self {
        #[allow(deprecated)]
        val.into_string()
    }
}

impl From<FastStr> for Bytes {
    #[inline]
    fn from(val: FastStr) -> Self {
        val.into_bytes()
    }
}

impl PartialEq<FastStr> for FastStr {
    #[inline]
    fn eq(&self, other: &FastStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for FastStr {}

impl PartialEq<str> for FastStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<FastStr> for str {
    #[inline]
    fn eq(&self, other: &FastStr) -> bool {
        other == self
    }
}

impl<'a> PartialEq<&'a str> for FastStr {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<FastStr> for &'a str {
    #[inline]
    fn eq(&self, other: &FastStr) -> bool {
        *self == other
    }
}

impl PartialEq<String> for FastStr {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<FastStr> for String {
    #[inline]
    fn eq(&self, other: &FastStr) -> bool {
        other == self
    }
}

impl<'a> PartialEq<&'a String> for FastStr {
    #[inline]
    fn eq(&self, other: &&'a String) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<FastStr> for &'a String {
    #[inline]
    fn eq(&self, other: &FastStr) -> bool {
        *self == other
    }
}

impl Ord for FastStr {
    #[inline]
    fn cmp(&self, other: &FastStr) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for FastStr {
    #[inline]
    fn partial_cmp(&self, other: &FastStr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl hash::Hash for FastStr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl fmt::Debug for FastStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for FastStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl iter::FromIterator<char> for FastStr {
    #[inline]
    fn from_iter<I: iter::IntoIterator<Item = char>>(iter: I) -> FastStr {
        let iter = iter.into_iter();
        Self::from_char_iter(iter)
    }
}

fn build_from_str_iter<T>(mut iter: impl Iterator<Item = T>) -> FastStr
where
    T: AsRef<str>,
    String: iter::Extend<T>,
{
    let mut len = 0;
    let mut buf = [0u8; INLINE_CAP];
    while let Some(slice) = iter.next() {
        let slice = slice.as_ref();
        let size = slice.len();
        if size + len > INLINE_CAP {
            let mut s = String::with_capacity(size + len);
            s.push_str(unsafe { core::str::from_utf8_unchecked(&buf[..len]) });
            s.push_str(slice);
            s.extend(iter);
            return FastStr(Repr::Bytes(Bytes::from(s)));
        }
        buf[len..][..size].copy_from_slice(slice.as_bytes());
        len += size;
    }
    FastStr(Repr::Inline { len, buf })
}

impl iter::FromIterator<String> for FastStr {
    #[inline]
    fn from_iter<I: iter::IntoIterator<Item = String>>(iter: I) -> FastStr {
        build_from_str_iter(iter.into_iter())
    }
}

impl<'a> iter::FromIterator<&'a String> for FastStr {
    #[inline]
    fn from_iter<I: iter::IntoIterator<Item = &'a String>>(iter: I) -> FastStr {
        FastStr::from_iter(iter.into_iter().map(|x| x.as_str()))
    }
}

impl<'a> iter::FromIterator<&'a str> for FastStr {
    #[inline]
    fn from_iter<I: iter::IntoIterator<Item = &'a str>>(iter: I) -> FastStr {
        build_from_str_iter(iter.into_iter())
    }
}

impl Borrow<str> for FastStr {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for FastStr {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<FastStr, Self::Err> {
        Ok(FastStr::new(s))
    }
}

// We need to wait for specilization to be stable to implement this.
// impl<T> From<T> for FastStr
// where
//     T: AsRef<str>,
// {
//     #[inline]
//     fn from(text: T) -> Self {
//         Self::new(text)
//     }
// }

impl From<Arc<str>> for FastStr {
    #[inline]
    fn from(val: Arc<str>) -> Self {
        Self::from_arc_str(val)
    }
}

impl From<String> for FastStr {
    #[inline]
    fn from(val: String) -> Self {
        Self::from_string(val)
    }
}

impl From<Arc<String>> for FastStr {
    #[inline]
    fn from(val: Arc<String>) -> Self {
        Self::from_arc_string(val)
    }
}

impl From<&'static str> for FastStr {
    #[inline]
    fn from(val: &'static str) -> Self {
        Self::from_static_str(val)
    }
}

impl From<Cow<'static, str>> for FastStr {
    #[inline]
    fn from(val: Cow<'static, str>) -> Self {
        match val {
            Cow::Borrowed(s) => Self::from_static_str(s),
            Cow::Owned(s) => Self::from_string(s),
        }
    }
}

const INLINE_CAP: usize = 24;

#[derive(Clone)]
enum Repr {
    Empty,
    Bytes(Bytes),
    ArcStr(Arc<str>),
    ArcString(Arc<String>),
    StaticStr(&'static str),
    Inline { len: usize, buf: [u8; INLINE_CAP] },
}

impl Repr {
    #[inline]
    fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        let text = text.as_ref();
        if text.is_empty() {
            return Self::Empty;
        }
        {
            let len = text.len();
            if len <= INLINE_CAP {
                // Safety: we have checked the length of text <= `INLINE_CAP`.
                return unsafe { Self::new_inline_impl(text) };
            }
        }

        Self::Bytes(Bytes::copy_from_slice(text.as_bytes()))
    }

    fn new_inline(s: &str) -> Self {
        if s.len() > INLINE_CAP {
            panic!("[FastStr] string is too long to inline");
        }
        // Safety: we have checked the length of s <= `INLINE_CAP`.
        unsafe { Self::new_inline_impl(s) }
    }

    /// # Safety
    ///
    /// The length of `s` must be <= `INLINE_CAP`.
    unsafe fn new_inline_impl(s: &str) -> Self {
        #[allow(invalid_value, clippy::uninit_assumed_init)]
        let mut inl = Self::Inline {
            len: s.len(),
            buf: MaybeUninit::uninit().assume_init(),
        };
        match inl {
            Self::Inline {
                ref mut len,
                ref mut buf,
            } => {
                // We can't guarantee if it's nonoverlapping here, so we can only use std::ptr::copy.
                std::ptr::copy(s.as_ptr(), buf.as_mut_ptr(), s.len());
                *len = s.len();
            }
            _ => unreachable_unchecked(),
        }
        inl
    }

    #[inline]
    const fn empty() -> Self {
        Self::Empty
    }

    #[inline]
    fn from_arc_str(s: Arc<str>) -> Self {
        Self::ArcStr(s)
    }

    #[inline]
    fn from_string(s: String) -> Self {
        let v = s.into_bytes();
        // Safety: s is a `String`, thus we can assume it's valid utf-8
        unsafe { Self::from_bytes_unchecked(v.into()) }
    }

    #[inline]
    fn from_arc_string(s: Arc<String>) -> Self {
        match Arc::try_unwrap(s) {
            Ok(s) => Self::from_string(s),
            Err(s) => Self::ArcString(s),
        }
    }

    /// Safety: the caller must guarantee that the bytes `v` are valid UTF-8.
    #[inline]
    unsafe fn from_bytes_unchecked(bytes: Bytes) -> Self {
        Self::Bytes(bytes)
    }

    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Bytes(bytes) => bytes.len(),
            Self::ArcStr(arc_str) => arc_str.len(),
            Self::ArcString(arc_string) => arc_string.len(),
            Self::StaticStr(s) => s.len(),
            Self::Inline { len, .. } => *len,
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Bytes(bytes) => bytes.is_empty(),
            Self::ArcStr(arc_str) => arc_str.is_empty(),
            Self::ArcString(arc_string) => arc_string.is_empty(),
            Self::StaticStr(s) => s.is_empty(),
            Self::Inline { len, .. } => *len == 0,
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        match self {
            Self::Empty => "",
            // Safety: this is guaranteed by the user when creating the `FastStr`.
            Self::Bytes(bytes) => unsafe { std::str::from_utf8_unchecked(bytes) },
            Self::ArcStr(arc_str) => arc_str,
            Self::ArcString(arc_string) => arc_string,
            Self::StaticStr(s) => s,
            Self::Inline { len, buf } => unsafe { std::str::from_utf8_unchecked(&buf[..*len]) },
        }
    }

    #[inline]
    #[deprecated]
    fn into_string(self) -> String {
        match self {
            Self::Empty => String::new(),
            Self::Bytes(bytes) => unsafe { String::from_utf8_unchecked(bytes.into()) },
            Self::ArcStr(arc_str) => arc_str.to_string(),
            Self::ArcString(arc_string) => {
                Arc::try_unwrap(arc_string).unwrap_or_else(|arc| (*arc).clone())
            }
            Self::StaticStr(s) => s.to_string(),
            Self::Inline { len, buf } => unsafe {
                String::from_utf8_unchecked(buf[..len].to_vec())
            },
        }
    }

    #[inline]
    fn into_bytes(self) -> Bytes {
        match self {
            Self::Empty => Bytes::new(),
            Self::Bytes(bytes) => bytes,
            Self::ArcStr(arc_str) => Bytes::from(arc_str.as_bytes().to_vec()),
            Self::ArcString(arc_string) => {
                Bytes::from(Arc::try_unwrap(arc_string).unwrap_or_else(|arc| (*arc).clone()))
            }
            Self::StaticStr(s) => Bytes::from_static(s.as_bytes()),
            Self::Inline { len, buf } => Bytes::from(buf[..len].to_vec()),
        }
    }

    #[inline]
    fn slice_ref(&self, subset: &[u8]) -> Self {
        if subset.is_empty() {
            return Self::Empty;
        }
        let bytes_p = self.as_ref().as_ptr() as usize;
        let bytes_len = self.len();

        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();

        assert!(
            sub_p >= bytes_p,
            "subset pointer ({:p}) is smaller than self pointer ({:p})",
            subset.as_ptr(),
            self.as_ref().as_ptr(),
        );
        assert!(
            sub_p + sub_len <= bytes_p + bytes_len,
            "subset is out of bounds: self = ({:p}, {}), subset = ({:p}, {})",
            self.as_ref().as_ptr(),
            bytes_len,
            subset.as_ptr(),
            sub_len,
        );

        let sub_offset = sub_p - bytes_p;
        match self {
            Repr::Empty => panic!("invalid slice ref, self is empty but subset is not"),
            Repr::Bytes(b) => Self::Bytes(b.slice_ref(subset)),
            Repr::ArcStr(s) => Self::Bytes(Bytes::copy_from_slice(
                s[sub_offset..sub_offset + sub_len].as_bytes(),
            )),
            Repr::ArcString(s) => Self::Bytes(Bytes::copy_from_slice(
                s[sub_offset..sub_offset + sub_len].as_bytes(),
            )),
            Repr::StaticStr(s) => Self::StaticStr(unsafe {
                std::str::from_utf8_unchecked(&s.as_bytes()[sub_offset..sub_offset + sub_len])
            }),
            Repr::Inline { len: _, buf } => Self::Inline {
                len: sub_len,
                buf: {
                    let mut new_buf = [0; INLINE_CAP];
                    new_buf[..sub_len].copy_from_slice(&buf[sub_offset..sub_offset + sub_len]);
                    new_buf
                },
            },
        }
    }
}

impl AsRef<[u8]> for Repr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Empty => &[],
            Self::Bytes(bytes) => bytes.as_ref(),
            Self::ArcStr(arc_str) => arc_str.as_bytes(),
            Self::ArcString(arc_string) => arc_string.as_bytes(),
            Self::StaticStr(s) => s.as_bytes(),
            Self::Inline { len, buf } => &buf[..*len],
        }
    }
}
#[cfg(feature = "redis")]
mod redis;

#[cfg(feature = "serde")]
mod serde;
