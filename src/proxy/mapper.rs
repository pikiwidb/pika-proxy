use lazy_static::*;
use std::collections::HashMap;

const MAX_OP_STR_LEN: usize = 64;

#[derive(Clone, Copy)]
pub struct OpFlag(u32);

impl OpFlag {
    fn is_not_allowed(&self) -> bool {
        (self.0 & FLAG_NOT_ALLOW) != 0
    }

    fn is_read_only(&self) -> bool {
        const MASK: u32 = FLAG_WRITE | FLAG_MAY_WRITE;
        (self.0 & MASK) == 0
    }

    fn is_master_only(&self) -> bool {
        const MASK: u32 = FLAG_WRITE | FLAG_MAY_WRITE | FLAG_MASTER_ONLY;
        (self.0 & MASK) != 0
    }
}

struct OpInfo {
    name: String,
    flag: OpFlag,
}

const FLAG_WRITE: u32 = 1 << 0;
const FLAG_MASTER_ONLY: u32 = 1 << 1;
const FLAG_MAY_WRITE: u32 = 1 << 2;
const FLAG_NOT_ALLOW: u32 = 1 << 3;

lazy_static! {
    static ref OP_TABLE: HashMap<&'static str, OpInfo> = {
        let mut table = HashMap::new();
        let op_infos = [
            ("APPEND", OpFlag(FLAG_WRITE)),
            ("ASKING", OpFlag(FLAG_NOT_ALLOW)),
            ("AUTH", OpFlag(0)),
            ("BGREWRITEAOF", OpFlag(FLAG_NOT_ALLOW)),
            ("BGSAVE", OpFlag(FLAG_NOT_ALLOW)),
            ("BITCOUNT", OpFlag(0)),
            ("BITFIELD", OpFlag(FLAG_WRITE)),
            ("BITOP", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("BITPOS", OpFlag(0)),
            ("BLPOP", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("BRPOP", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("BRPOPLPUSH", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("CLIENT", OpFlag(FLAG_NOT_ALLOW)),
            ("CLUSTER", OpFlag(FLAG_NOT_ALLOW)),
            ("COMMAND", OpFlag(0)),
            ("CONFIG", OpFlag(FLAG_NOT_ALLOW)),
            ("DBSIZE", OpFlag(FLAG_NOT_ALLOW)),
            ("DEBUG", OpFlag(FLAG_NOT_ALLOW)),
            ("DECR", OpFlag(FLAG_WRITE)),
            ("DECRBY", OpFlag(FLAG_WRITE)),
            ("DEL", OpFlag(FLAG_WRITE)),
            ("DISCARD", OpFlag(FLAG_NOT_ALLOW)),
            ("DUMP", OpFlag(0)),
            ("ECHO", OpFlag(0)),
            ("EVAL", OpFlag(FLAG_WRITE)),
            ("EVALSHA", OpFlag(FLAG_WRITE)),
            ("EXEC", OpFlag(FLAG_NOT_ALLOW)),
            ("EXISTS", OpFlag(0)),
            ("EXPIRE", OpFlag(FLAG_WRITE)),
            ("EXPIREAT", OpFlag(FLAG_WRITE)),
            ("FLUSHALL", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("FLUSHDB", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("GEOADD", OpFlag(FLAG_WRITE)),
            ("GEODIST", OpFlag(0)),
            ("GEOHASH", OpFlag(0)),
            ("GEOPOS", OpFlag(0)),
            ("GEORADIUS", OpFlag(FLAG_WRITE)),
            ("GEORADIUSBYMEMBER", OpFlag(FLAG_WRITE)),
            ("GET", OpFlag(0)),
            ("GETBIT", OpFlag(0)),
            ("GETRANGE", OpFlag(0)),
            ("GETSET", OpFlag(FLAG_WRITE)),
            ("HDEL", OpFlag(FLAG_WRITE)),
            ("HEXISTS", OpFlag(0)),
            ("HGET", OpFlag(0)),
            ("HGETALL", OpFlag(0)),
            ("HINCRBY", OpFlag(FLAG_WRITE)),
            ("HINCRBYFLOAT", OpFlag(FLAG_WRITE)),
            ("HKEYS", OpFlag(0)),
            ("HLEN", OpFlag(0)),
            ("HMGET", OpFlag(0)),
            ("HMSET", OpFlag(FLAG_WRITE)),
            ("HOST:", OpFlag(FLAG_NOT_ALLOW)),
            ("HSCAN", OpFlag(FLAG_MASTER_ONLY)),
            ("HSET", OpFlag(FLAG_WRITE)),
            ("HSETNX", OpFlag(FLAG_WRITE)),
            ("HSTRLEN", OpFlag(0)),
            ("HVALS", OpFlag(0)),
            ("INCR", OpFlag(FLAG_WRITE)),
            ("INCRBY", OpFlag(FLAG_WRITE)),
            ("INCRBYFLOAT", OpFlag(FLAG_WRITE)),
            ("INFO", OpFlag(0)),
            ("KEYS", OpFlag(FLAG_NOT_ALLOW)),
            ("LASTSAVE", OpFlag(FLAG_NOT_ALLOW)),
            ("LATENCY", OpFlag(FLAG_NOT_ALLOW)),
            ("LINDEX", OpFlag(0)),
            ("LINSERT", OpFlag(FLAG_WRITE)),
            ("LLEN", OpFlag(0)),
            ("LPOP", OpFlag(FLAG_WRITE)),
            ("LPUSH", OpFlag(FLAG_WRITE)),
            ("LPUSHX", OpFlag(FLAG_WRITE)),
            ("LRANGE", OpFlag(0)),
            ("LREM", OpFlag(FLAG_WRITE)),
            ("LSET", OpFlag(FLAG_WRITE)),
            ("LTRIM", OpFlag(FLAG_WRITE)),
            ("MGET", OpFlag(0)),
            ("MIGRATE", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("MONITOR", OpFlag(FLAG_NOT_ALLOW)),
            ("MOVE", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("MSET", OpFlag(FLAG_WRITE)),
            ("MSETNX", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("MULTI", OpFlag(FLAG_NOT_ALLOW)),
            ("OBJECT", OpFlag(FLAG_NOT_ALLOW)),
            ("PERSIST", OpFlag(FLAG_WRITE)),
            ("PEXPIRE", OpFlag(FLAG_WRITE)),
            ("PEXPIREAT", OpFlag(FLAG_WRITE)),
            ("PFADD", OpFlag(FLAG_WRITE)),
            ("PFCOUNT", OpFlag(0)),
            ("PFDEBUG", OpFlag(FLAG_WRITE)),
            ("PFMERGE", OpFlag(FLAG_WRITE)),
            ("PFSELFTEST", OpFlag(0)),
            ("PING", OpFlag(0)),
            ("POST", OpFlag(FLAG_NOT_ALLOW)),
            ("PSETEX", OpFlag(FLAG_WRITE)),
            ("PSUBSCRIBE", OpFlag(FLAG_NOT_ALLOW)),
            ("PSYNC", OpFlag(FLAG_NOT_ALLOW)),
            ("PTTL", OpFlag(0)),
            ("PUBLISH", OpFlag(FLAG_NOT_ALLOW)),
            ("PUBSUB", OpFlag(0)),
            ("PUNSUBSCRIBE", OpFlag(FLAG_NOT_ALLOW)),
            ("QUIT", OpFlag(0)),
            ("RANDOMKEY", OpFlag(FLAG_NOT_ALLOW)),
            ("READONLY", OpFlag(FLAG_NOT_ALLOW)),
            ("READWRITE", OpFlag(FLAG_NOT_ALLOW)),
            ("RENAME", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("RENAMENX", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("REPLCONF", OpFlag(FLAG_NOT_ALLOW)),
            ("RESTORE", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("RESTORE-ASKING", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("ROLE", OpFlag(0)),
            ("RPOP", OpFlag(FLAG_WRITE)),
            ("RPOPLPUSH", OpFlag(FLAG_WRITE)),
            ("RPUSH", OpFlag(FLAG_WRITE)),
            ("RPUSHX", OpFlag(FLAG_WRITE)),
            ("SADD", OpFlag(FLAG_WRITE)),
            ("SAVE", OpFlag(FLAG_NOT_ALLOW)),
            ("SCAN", OpFlag(FLAG_MASTER_ONLY | FLAG_NOT_ALLOW)),
            ("SCARD", OpFlag(0)),
            ("SCRIPT", OpFlag(FLAG_NOT_ALLOW)),
            ("SDIFF", OpFlag(0)),
            ("SDIFFSTORE", OpFlag(FLAG_WRITE)),
            ("SELECT", OpFlag(0)),
            ("SET", OpFlag(FLAG_WRITE)),
            ("SETBIT", OpFlag(FLAG_WRITE)),
            ("SETEX", OpFlag(FLAG_WRITE)),
            ("SETNX", OpFlag(FLAG_WRITE)),
            ("SETRANGE", OpFlag(FLAG_WRITE)),
            ("SHUTDOWN", OpFlag(FLAG_NOT_ALLOW)),
            ("SINTER", OpFlag(0)),
            ("SINTERSTORE", OpFlag(FLAG_WRITE)),
            ("SISMEMBER", OpFlag(0)),
            ("SLAVEOF", OpFlag(FLAG_NOT_ALLOW)),
            ("SLOTSCHECK", OpFlag(FLAG_NOT_ALLOW)),
            ("SLOTSDEL", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSHASHKEY", OpFlag(0)),
            ("SLOTSINFO", OpFlag(FLAG_MASTER_ONLY)),
            ("SLOTSMAPPING", OpFlag(0)),
            ("SLOTSMGRTONE", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSMGRTSLOT", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSMGRTTAGONE", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSMGRTTAGSLOT", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSRESTORE", OpFlag(FLAG_WRITE)),
            ("SLOTSMGRTONE-ASYNC", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSMGRTSLOT-ASYNC", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            ("SLOTSMGRTTAGONE-ASYNC", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            (
                "SLOTSMGRTTAGSLOT-ASYNC",
                OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW),
            ),
            ("SLOTSMGRT-ASYNC-FENCE", OpFlag(FLAG_NOT_ALLOW)),
            ("SLOTSMGRT-ASYNC-CANCEL", OpFlag(FLAG_NOT_ALLOW)),
            ("SLOTSMGRT-ASYNC-STATUS", OpFlag(FLAG_NOT_ALLOW)),
            (
                "SLOTSMGRT-EXEC-WRAPPER",
                OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW),
            ),
            ("SLOTSRESTORE-ASYNC", OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW)),
            (
                "SLOTSRESTORE-ASYNC-AUTH",
                OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW),
            ),
            (
                "SLOTSRESTORE-ASYNC-ACK",
                OpFlag(FLAG_WRITE | FLAG_NOT_ALLOW),
            ),
            ("SLOTSSCAN", OpFlag(FLAG_MASTER_ONLY)),
            ("SLOWLOG", OpFlag(FLAG_NOT_ALLOW)),
            ("SMEMBERS", OpFlag(0)),
            ("SMOVE", OpFlag(FLAG_WRITE)),
            ("SORT", OpFlag(FLAG_WRITE)),
            ("SPOP", OpFlag(FLAG_WRITE)),
            ("SRANDMEMBER", OpFlag(0)),
            ("SREM", OpFlag(FLAG_WRITE)),
            ("SSCAN", OpFlag(FLAG_MASTER_ONLY)),
            ("STRLEN", OpFlag(0)),
            ("SUBSCRIBE", OpFlag(FLAG_NOT_ALLOW)),
            ("SUBSTR", OpFlag(0)),
            ("SUNION", OpFlag(0)),
            ("SUNIONSTORE", OpFlag(FLAG_WRITE)),
            ("SYNC", OpFlag(FLAG_NOT_ALLOW)),
            ("TIME", OpFlag(FLAG_NOT_ALLOW)),
            ("TOUCH", OpFlag(FLAG_WRITE)),
            ("TTL", OpFlag(0)),
            ("TYPE", OpFlag(0)),
            ("UNSUBSCRIBE", OpFlag(FLAG_NOT_ALLOW)),
            ("UNWATCH", OpFlag(FLAG_NOT_ALLOW)),
            ("WAIT", OpFlag(FLAG_NOT_ALLOW)),
            ("WATCH", OpFlag(FLAG_NOT_ALLOW)),
            ("ZADD", OpFlag(FLAG_WRITE)),
            ("ZCARD", OpFlag(0)),
            ("ZCOUNT", OpFlag(0)),
            ("ZINCRBY", OpFlag(FLAG_WRITE)),
            ("ZINTERSTORE", OpFlag(FLAG_WRITE)),
            ("ZLEXCOUNT", OpFlag(0)),
            ("ZPOPMAX", OpFlag(FLAG_WRITE)),
            ("ZPOPMIN", OpFlag(FLAG_WRITE)),
            ("ZRANGE", OpFlag(0)),
            ("ZRANGEBYLEX", OpFlag(0)),
            ("ZRANGEBYSCORE", OpFlag(0)),
            ("ZRANK", OpFlag(0)),
            ("ZREM", OpFlag(FLAG_WRITE)),
            ("ZREMRANGEBYLEX", OpFlag(FLAG_WRITE)),
            ("ZREMRANGEBYRANK", OpFlag(FLAG_WRITE)),
            ("ZREMRANGEBYSCORE", OpFlag(FLAG_WRITE)),
            ("ZREVRANGE", OpFlag(0)),
            ("ZREVRANGEBYLEX", OpFlag(0)),
            ("ZREVRANGEBYSCORE", OpFlag(0)),
            ("ZREVRANK", OpFlag(0)),
            ("ZSCAN", OpFlag(FLAG_MASTER_ONLY)),
            ("ZSCORE", OpFlag(0)),
            ("ZUNIONSTORE", OpFlag(FLAG_WRITE)),
        ];

        for (name, flag) in op_infos.iter() {
            table.insert(
                *name,
                OpInfo {
                    name: name.to_string(),
                    flag: *flag,
                },
            );
        }

        table
    };
}

#[derive(Debug)]
struct OpError(String);

fn get_op_info(op: &[u8]) -> Result<(&str, OpFlag), OpError> {
    if op.is_empty() {
        return Err(OpError("bad multi-bulk for command".to_string()));
    }

    let op_str = std::str::from_utf8(op)
        .map_err(|_| OpError("bad command length, too short or too long".to_string()))?;

    if let Some(op_info) = OP_TABLE.get(op_str) {
        Ok((&op_info.name, op_info.flag))
    } else {
        Ok((op_str, OpFlag(FLAG_MAY_WRITE)))
    }
}

fn hash(key: &[u8]) -> u32 {
    const TAG_BEG: u8 = b'{';
    const TAG_END: u8 = b'}';

    let mut hasher = crc32fast::Hasher::new();
    let mut has_tag = false;

    for &byte in key.iter() {
        if byte == TAG_BEG {
            has_tag = true;
            continue;
        }

        if has_tag && byte == TAG_END {
            break;
        }

        hasher.update(&[byte]);
    }

    hasher.finalize()
}

fn get_hash_key<'a>(multi: &'a [&'a [u8]], op_str: &'a str) -> Option<&'a [u8]> {
    let mut index = 1;
    match op_str {
        "ZINTERSTORE" | "ZUNIONSTORE" | "EVAL" | "EVALSHA" => index = 3,
        _ => (),
    }

    multi.get(index).copied()
}

mod tests {
    use super::*;

    #[test]
    fn test_is_not_allowed() {
        let flag = OpFlag(FLAG_NOT_ALLOW);
        assert_eq!(flag.is_not_allowed(), true);

        let flag = OpFlag(FLAG_WRITE);
        assert_eq!(flag.is_not_allowed(), false);

        let flag = OpFlag(FLAG_NOT_ALLOW | FLAG_WRITE);
        assert_eq!(flag.is_not_allowed(), true);
    }
}
