use redis::{Client, RedisError, RedisWrite, ToRedisArgs};

use crate::model::Planet;

pub fn create_redis_client(redis_uri: String) -> Result<Client, RedisError> {
    let client = Client::open(redis_uri)?;

    Ok(client)
}

impl ToRedisArgs for &Planet {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let result = serde_json::to_string(self).expect("Failed to serialize Planet as string");

        out.write_arg_fmt(result)
    }
}
