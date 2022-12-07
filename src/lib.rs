use bytes::{Bytes, BytesMut};
use std::{
    borrow::Borrow, cmp::Ordering, convert::Infallible, fmt, hash, iter, ops::Deref, str::FromStr,
    string::FromUtf8Error, sync::Arc,
};

#[derive(Clone)]
pub struct FastStr(Repr);

impl FastStr {
    pub const fn new_inline(s: &str) -> Self {
        if s.len() > INLINE_CAP {
            panic!("[FastStr] string is too long to inline");
        }
        let mut buf = [0; INLINE_CAP];
        let mut i = 0;
        while i < s.len() {
            buf[i] = s.as_bytes()[i];
            i += 1
        }
        Self(Repr::Inline {
            len: s.len() as u8,
            buf,
        })
    }

    #[inline]
    pub fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        Self(Repr::new(text))
    }

    #[inline]
    pub fn from_string(s: String) -> Self {
        Self::from_arc_string(Arc::new(s))
    }

    #[inline]
    pub fn from_arc_string(s: Arc<String>) -> Self {
        Self(Repr::from_arc_string(s))
    }

    /// # Safety
    ///
    /// `b` must be valid UTF-8
    #[inline]
    pub unsafe fn from_bytes_unchecked(b: Bytes) -> Self {
        Self(Repr::from_bytes_unchecked(b))
    }

    /// # Safety
    ///
    /// `b` must be valid UTF-8
    pub unsafe fn from_bytes_mut_unchecked(b: BytesMut) -> Self {
        let v = b.into();
        let s = unsafe { String::from_utf8_unchecked(v) };
        Self::from_string(s)
    }

    pub fn from_bytes_mut(b: BytesMut) -> Result<Self, FromUtf8Error> {
        let v = b.into();
        let s = String::from_utf8(v)?;
        Ok(Self::from_string(s))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline(always)]
    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn from_char_iter<I: iter::Iterator<Item = char>>(mut iter: I) -> Self {
        let (min_size, _) = iter.size_hint();
        if min_size > INLINE_CAP {
            let s: String = iter.collect();
            return Self(Repr::from_arc_string(Arc::new(s)));
        }
        let mut len = 0;
        let mut buf = [0u8; INLINE_CAP];
        while let Some(ch) = iter.next() {
            let size = ch.len_utf8();
            if size + len > INLINE_CAP {
                let (min_remaining, _) = iter.size_hint();
                let mut s = String::with_capacity(size + len + min_remaining);
                s.push_str(core::str::from_utf8(&buf[..len]).unwrap());
                s.push(ch);
                s.extend(iter);
                return Self(Repr::ArcStr(s.into_boxed_str().into()));
            }
            ch.encode_utf8(&mut buf[len..]);
            len += size;
        }
        Self(Repr::Inline {
            len: len as u8,
            buf,
        })
    }
}

impl Default for FastStr {
    #[inline]
    fn default() -> Self {
        Self::new("")
    }
}

impl AsRef<[u8]> for FastStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
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
        val.into_string()
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
            s.push_str(core::str::from_utf8(&buf[..len]).unwrap());
            s.push_str(slice);
            s.extend(iter);
            return FastStr(Repr::from_arc_string(Arc::new(s)));
        }
        buf[len..][..size].copy_from_slice(slice.as_bytes());
        len += size;
    }
    FastStr(Repr::Inline {
        len: len as u8,
        buf,
    })
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

impl<T> From<T> for FastStr
where
    T: AsRef<str>,
{
    #[inline]
    fn from(text: T) -> Self {
        Self::new(text)
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
        Ok(FastStr::from(s))
    }
}

const INLINE_CAP: usize = 22;

#[derive(Clone)]
enum Repr {
    Bytes(Bytes),
    ArcStr(Arc<str>),
    ArcString(Arc<String>),
    Inline { len: u8, buf: [u8; INLINE_CAP] },
}

impl Repr {
    #[inline]
    fn new<T>(text: T) -> Self
    where
        T: AsRef<str>,
    {
        {
            let text = text.as_ref();

            let len = text.len();
            if len <= INLINE_CAP {
                let mut buf = [0; INLINE_CAP];
                buf[..len].copy_from_slice(text.as_bytes());
                return Self::Inline {
                    len: len as u8,
                    buf,
                };
            }
        }

        Self::ArcStr(text.as_ref().into())
    }

    #[inline]
    fn from_arc_string(s: Arc<String>) -> Self {
        Self::ArcString(s)
    }

    /// Safety: the caller must guarantee that the bytes `v` are valid UTF-8.
    #[inline]
    unsafe fn from_bytes_unchecked(bytes: Bytes) -> Self {
        Self::Bytes(bytes)
    }

    #[inline]
    fn len(&self) -> usize {
        match self {
            Self::Bytes(bytes) => bytes.len(),
            Self::ArcStr(arc_str) => arc_str.len(),
            Self::ArcString(arc_string) => arc_string.len(),
            Self::Inline { len, .. } => *len as usize,
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            Self::Bytes(bytes) => bytes.is_empty(),
            Self::ArcStr(arc_str) => arc_str.is_empty(),
            Self::ArcString(arc_string) => arc_string.is_empty(),
            Self::Inline { len, .. } => *len == 0,
        }
    }

    #[inline]
    fn as_str(&self) -> &str {
        match self {
            // Safety: this is guaranteed by the user when creating the `FastStr`.
            Self::Bytes(bytes) => unsafe { std::str::from_utf8_unchecked(bytes) },
            Self::ArcStr(arc_str) => arc_str,
            Self::ArcString(arc_string) => arc_string,
            Self::Inline { len, buf } => unsafe {
                std::str::from_utf8_unchecked(&buf[..*len as usize])
            },
        }
    }

    #[inline]
    fn into_string(self) -> String {
        match self {
            Self::Bytes(bytes) => unsafe { String::from_utf8_unchecked(bytes.into()) },
            Self::ArcStr(arc_str) => arc_str.to_string(),
            Self::ArcString(arc_string) => {
                Arc::try_unwrap(arc_string).unwrap_or_else(|arc| (*arc).clone())
            }
            Self::Inline { len, buf } => unsafe {
                String::from_utf8_unchecked(buf[..len as usize].to_vec())
            },
        }
    }
}

impl AsRef<[u8]> for Repr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Bytes(bytes) => bytes.as_ref(),
            Self::ArcStr(arc_str) => arc_str.as_bytes(),
            Self::ArcString(arc_string) => arc_string.as_bytes(),
            Self::Inline { len, buf } => &buf[..*len as usize],
        }
    }
}
