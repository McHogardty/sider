
use bytes::Bytes;
use command_macro::command;

use sider_command::RESPType;
use super::super::db::DB;


#[command(
    name = "exists",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn exists(args: Vec<RESPType<Bytes>>, db: &mut DB) -> RESPType<Bytes> {
    if args.len() < 1 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let mut total = 0;

    for a in args {
        let RESPType::BulkString(k) = a else {
            return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
        };

        total += db.exists(&k.into()) as i64;
    }

    RESPType::Integer(total)
}