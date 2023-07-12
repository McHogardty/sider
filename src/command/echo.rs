
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
pub fn echo<'a>(args: Vec<RESPType>, _: &mut DB) -> RESPType {
    if args.len() != 1 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let RESPType::BulkString(_)= &args[0] else {
        return RESPType::Error("wrong number of arguments".into());
    };

    args[0].clone()
}