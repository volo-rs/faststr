use core::fmt;

#[cfg(not(feature = "serde-unsafe"))]
use serde::de::Unexpected;
use serde::de::{Deserializer, Error, Visitor};

use crate::FastStr;

// https://github.com/serde-rs/serde/blob/629802f2abfd1a54a6072992888fea7ca5bc209f/serde/src/private/de.rs#L56-L125
fn fast_str<'de: 'a, 'a, D>(deserializer: D) -> Result<FastStr, D::Error>
where
    D: Deserializer<'de>,
{
    struct FastStrVisitor;

    impl<'a> Visitor<'a> for FastStrVisitor {
        type Value = FastStr;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(FastStr::new(v))
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(FastStr::new(v))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(FastStr::from(v))
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            #[cfg(feature = "serde-unsafe")]
            {
                Ok(unsafe { FastStr::new_u8_slice_unchecked(v) })
            }
            #[cfg(not(feature = "serde-unsafe"))]
            match FastStr::new_u8_slice(v) {
                Ok(s) => Ok(s),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }

        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            #[cfg(feature = "serde-unsafe")]
            {
                Ok(unsafe { FastStr::new_u8_slice_unchecked(v) })
            }
            #[cfg(not(feature = "serde-unsafe"))]
            match FastStr::new_u8_slice(v) {
                Ok(s) => Ok(s),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: Error,
        {
            #[cfg(not(feature = "serde-unsafe"))]
            simdutf8::basic::from_utf8(&v)
                .map_err(|_| Error::invalid_value(Unexpected::Bytes(&v), &self))?;
            // Safety: we have checked that v is valid utf-8
            Ok(unsafe { FastStr::from_vec_u8_unchecked(v) })
        }
    }

    deserializer.deserialize_string(FastStrVisitor)
}

impl serde::Serialize for FastStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for FastStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        fast_str(deserializer)
    }
}
