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
            #[cfg(feature = "redis-unsafe")]
            redis::Value::Data(bytes) => {
                Ok(unsafe { Self::new(std::str::from_utf8_unchecked(bytes)) })
            }
            #[cfg(not(feature = "redis-unsafe"))]
            redis::Value::Data(bytes) => Ok(Self::new(std::str::from_utf8(bytes)?)),
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

    fn from_owned_redis_value(v: redis::Value) -> redis::RedisResult<Self> {
        match v {
            #[cfg(feature = "redis-unsafe")]
            redis::Value::Data(bytes) => Ok(unsafe { Self::from_vec_u8_unchecked(bytes) }),
            #[cfg(not(feature = "redis-unsafe"))]
            redis::Value::Data(bytes) => Self::from_vec_u8(bytes)
                .map_err(|_| (redis::ErrorKind::TypeError, "Invalid UTF8").into()),
            redis::Value::Nil => Ok(Self::empty()),
            redis::Value::Int(v) => Ok(Self::new(itoa::Buffer::new().format(v))),
            redis::Value::Status(s) => Ok(Self::from_string(s)),
            redis::Value::Okay => Ok(Self::from_static_str("OK")),
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid response type",
            ))),
        }
    }
}
