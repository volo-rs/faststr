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
            redis::Value::Nil => Ok(Self::empty()),
            redis::Value::Int(v) => Ok(Self::new(itoa::Buffer::new().format(*v))),
            redis::Value::Okay => Ok(Self::from_static_str("OK")),
            #[cfg(feature = "redis-unsafe")]
            redis::Value::BulkString(v) => {
                Ok(unsafe { Self::new(std::str::from_utf8_unchecked(v)) })
            }
            #[cfg(not(feature = "redis-unsafe"))]
            redis::Value::BulkString(v) => Ok(Self::new(std::str::from_utf8(v)?)),
            redis::Value::SimpleString(v) => Ok(Self::new(v)),
            redis::Value::Double(v) => Ok(Self::new(ryu::Buffer::new().format(*v))),
            redis::Value::Boolean(v) => {
                Ok(Self::from_static_str(if *v { "true" } else { "false" }))
            }
            redis::Value::BigNumber(v) => Ok(Self::from_string(v.to_string())),
            e => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid response type",
                format!("{e:?}"),
            ))),
        }
    }

    fn from_owned_redis_value(v: redis::Value) -> redis::RedisResult<Self> {
        match v {
            #[cfg(feature = "redis-unsafe")]
            redis::Value::BulkString(v) => Ok(unsafe { Self::from_vec_u8_unchecked(v) }),
            #[cfg(not(feature = "redis-unsafe"))]
            redis::Value::BulkString(v) => Self::from_vec_u8(v)
                .map_err(|_| (redis::ErrorKind::TypeError, "Invalid UTF8").into()),
            redis::Value::Nil => Ok(Self::empty()),
            redis::Value::Int(v) => Ok(Self::new(itoa::Buffer::new().format(v))),
            redis::Value::Okay => Ok(Self::from_static_str("OK")),
            redis::Value::SimpleString(v) => Ok(Self::from_string(v)),
            redis::Value::Double(v) => Ok(Self::new(ryu::Buffer::new().format(v))),
            redis::Value::Boolean(v) => Ok(Self::from_static_str(if v { "true" } else { "false" })),
            redis::Value::BigNumber(v) => Ok(Self::from_string(v.to_string())),
            e => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Invalid response type",
                format!("{e:?}"),
            ))),
        }
    }
}
