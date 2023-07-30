use redis_utils::RedisResp;

pub struct Response {
    redis: RedisResp,
    id: u64,
}
