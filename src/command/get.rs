
use command_macro::command;

use sider_command::RESPType;

use crate::db::DBError;

use super::super::db::DB;


#[command(
    name = "get",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn get(args: Vec<RESPType>, db: &mut DB) -> RESPType {
    if args.len() != 1 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let RESPType::BulkString(key) = &args[0] else {
        return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
    };

    let Some(e) = db.get(key) else {
        return RESPType::Null;
    };

    match e.get_string() {
        Ok(s) => RESPType::BulkString(s.to_bytes().to_vec()),
        Err(DBError::WrongType) => RESPType::Error("wrong type".into()),
        _ => panic!()
    }
}