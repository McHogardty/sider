
use bytes::Bytes;
use command_macro::command;

use sider_command::RESPType;
use super::super::db::DB;


#[command(
    name = "echo",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn echo(mut args: Vec<RESPType<Bytes>>, _: &mut DB) -> RESPType<Bytes> {
    if args.len() != 1 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let v @ RESPType::BulkString(_) = args.remove(0) else {
        return RESPType::Error("wrong number of arguments".into());
    };

    v
}