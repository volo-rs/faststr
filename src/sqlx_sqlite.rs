use crate::FastStr;
use sqlx::{encode::IsNull, error::BoxDynError, Decode, Encode, Type};
use sqlx_sqlite::{Sqlite, SqliteTypeInfo, SqliteValueRef};

impl Type<Sqlite> for FastStr {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <str as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &<Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <str as Type<Sqlite>>::compatible(ty)
    }
}

impl<'r> Decode<'r, Sqlite> for FastStr {
    fn decode(value: <Sqlite as sqlx::Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        #[cfg(not(feature = "sqlx-sqlite-unsafe"))]
        {
            let b = <&[u8] as Decode<Sqlite>>::decode(value)?;
            return simdutf8::basic::from_utf8(b)
                .map_err(|e| Box::new(e) as BoxDynError)
                .map(FastStr::new);
        }
        #[cfg(feature = "sqlx-sqlite-unsafe")]
        unsafe {
            return <&[u8] as Decode<Sqlite>>::decode(value)
                .map(|b| FastStr::new(std::str::from_utf8_unchecked(b)));
        }
    }
}

impl Encode<'_, Sqlite> for FastStr {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'_>,
    ) -> Result<IsNull, BoxDynError> {
        <&str as Encode<Sqlite>>::encode(self.as_str(), buf)
    }
    fn size_hint(&self) -> usize {
        <&str as Encode<Sqlite>>::size_hint(&self.as_str())
    }
}
