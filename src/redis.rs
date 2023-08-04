impl redis::ToRedisArgs for crate::FastStr {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.as_bytes())
    }
}

impl redis::FromRedisValue for crate::FastStr {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match v {
            redis::Value::Data(bytes) => {
                Ok(unsafe { Self::new(std::str::from_utf8_unchecked(bytes)) })
            }
            redis::Value::Nil => Ok(Self::empty()),
            redis::Value::Int(v) => Ok(Self::new(itoa::Buffer::new().format(*v))),
            redis::Value::Status(s) => Ok(Self::new(s)),
            redis::Value::Okay => Ok(Self::from_static_str("OK")),
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid response type",
            ))),
        }
    }

    fn from_byte_vec(_vec: &[u8]) -> Option<Vec<Self>> {
        let s = unsafe { Self::from_u8_slice_unchecked(_vec) };
        Some(vec![s])
    }
}
