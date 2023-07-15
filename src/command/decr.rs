

use bytes::Bytes;
use command_macro::command;

use sider_command::RESPType;
use crate::db::{DBError, ExpiryFlag, ExistenceFlag};

use super::super::db::DB;


#[command(
    name = "decr",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn decr(mut args: Vec<RESPType<Bytes>>, db: &mut DB) -> RESPType<Bytes> {
    if args.len() != 1 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let RESPType::BulkString(key) = args.remove(0) else {
        return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
    };

    let Ok(e) = db.get_or_insert(key, ExpiryFlag::KeepTTL, ExistenceFlag::None) else {
        return RESPType::Error("error retrieving key".into());
    };

    let v = if e.is_nil() {
        e.set_string(0.into());
        e.get_mut_string().unwrap()
    } else {
        match e.get_mut_string() {
            Err(DBError::WrongType) => return RESPType::Error("wrong type".into()),
            Ok(s) => s,
            _ => panic!()
        }
    };

    match v.incr_by(-1) {
        Ok(i) => RESPType::Integer(i),
        Err(()) => RESPType::Error("not a valid integer".into()),
    }
}