use crate::FastStr;
use sqlx::{encode::IsNull, error::BoxDynError, Decode, Encode, Type};
use sqlx_postgres::{Postgres, PgTypeInfo, PgValueRef};



impl Type<Postgres> for FastStr {
    fn type_info() -> PgTypeInfo {
        <str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <str as Type<Postgres>>::compatible(ty)
    }
}
impl<'r> Decode<'r, Postgres> for FastStr {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        #[cfg(not(feature = "sqlx-postgres-unsafe"))]
        {
            let b = <&[u8] as Decode<Postgres>>::decode(value)?;
            return simdutf8::basic::from_utf8(b)
                .map_err(|e| Box::new(e) as BoxDynError)
                .map(FastStr::new);
        }
        #[cfg(feature = "sqlx-postgres-unsafe")]
        unsafe {
            return <&[u8] as Decode<Postgres>>::decode(value)
                .map(|b| FastStr::new(std::str::from_utf8_unchecked(b)));
        }
    }
}

impl Encode<'_, Postgres> for FastStr {
    fn encode_by_ref(
            &self,
            buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'_>,
        ) -> Result<IsNull, BoxDynError> {
        <&str as Encode<Postgres>>::encode(self.as_str(), buf)
    }
    fn size_hint(&self) -> usize {
        <&str as Encode<Postgres>>::size_hint(&self.as_str())
    }
}