use sqlx::{encode::IsNull, error::BoxDynError, Decode, Encode, Type};
use sqlx_mysql::{MySql, MySqlTypeInfo, MySqlValueRef};

use crate::FastStr;

impl Type<MySql> for FastStr {
    fn type_info() -> MySqlTypeInfo {
        <str as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <str as Type<MySql>>::compatible(ty)
    }
}

impl<'r> Decode<'r, MySql> for FastStr {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        #[cfg(not(feature = "sqlx-mysql-unsafe"))]
        {
            let b = <&[u8] as Decode<MySql>>::decode(value)?;
            return simdutf8::basic::from_utf8(b)
                .map_err(|e| Box::new(e) as BoxDynError)
                .map(FastStr::new);
        }
        #[cfg(feature = "sqlx-mysql-unsafe")]
        unsafe {
            return <&[u8] as Decode<MySql>>::decode(value)
                .map(|b| FastStr::new(std::str::from_utf8_unchecked(b)));
        }
    }
}

impl Encode<'_, MySql> for FastStr {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        <&str as Encode<MySql>>::encode(self.as_str(), buf)
    }
    fn size_hint(&self) -> usize {
        <&str as Encode<MySql>>::size_hint(&self.as_str())
    }
}
