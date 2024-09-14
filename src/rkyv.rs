#[cfg(not(feature = "std"))]
use alloc::string::String;

use rancor::{Fallible, Source};
use rkyv::{
    ser::{Allocator, Writer},
    string::{ArchivedString, StringResolver},
    *,
};

use super::FastStr;

impl Archive for FastStr {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedString::resolve_from_str(self, resolver, out);
    }
}

impl<S> Serialize<S> for FastStr
where
    S: Fallible + Allocator + Writer + ?Sized,
    S::Error: Source,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(self, serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<FastStr, D> for ArchivedString {
    fn deserialize(&self, _deserializer: &mut D) -> Result<FastStr, D::Error> {
        Ok(FastStr::new(self.as_str()))
    }
}

impl PartialEq<FastStr> for ArchivedString {
    fn eq(&self, other: &FastStr) -> bool {
        other.as_str() == self.as_str()
    }
}

impl PartialOrd<FastStr> for ArchivedString {
    fn partial_cmp(&self, other: &FastStr) -> Option<::core::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}
