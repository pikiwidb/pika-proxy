use redis_utils::command::RedisCmd;

pub struct Request {
    redis: RedisCmd,
    id: u64,
}
