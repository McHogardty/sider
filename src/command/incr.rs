


use bytes::Bytes;
use command_macro::command;

use sider_command::RESPType;
use crate::db::DBError;

use super::super::db::DB;



#[command(
    name = "incr",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn incr(args: Vec<RESPType<Vec<u8>>>, db: &mut DB) -> RESPType<Bytes> {
    if args.len() != 1 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let RESPType::BulkString(key) = &args[0] else {
        return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
    };

    let Some(e) = db.get_mut(&key) else {
        return RESPType::Error("key not set".into());
    };

    let v = match e.get_mut_string() {
        Err(DBError::WrongType) => return RESPType::Error("wrong type".into()),
        Ok(s) => s,
        _ => panic!()
    };

    match v.incr_by(1) {
        Ok(i) => RESPType::Integer(i),
        Err(()) => RESPType::Error("value at key is not a valid integer".into())
    }
}