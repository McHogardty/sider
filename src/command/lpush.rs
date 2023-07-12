

use std::collections::VecDeque;

use command_macro::command;

use sider_command::RESPType;
use crate::db::{ExpiryFlag, ExistenceFlag, DBEntry};

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
pub fn lpush(args: Vec<RESPType>, db: &mut DB) -> RESPType {
    if args.len() < 2 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let RESPType::BulkString(key) = &args[0] else {
        return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
    };

    let entry = db.get_or_insert(key.clone(), ExpiryFlag::KeepTTL, ExistenceFlag::None).unwrap();

    let l = match entry {
        DBEntry::List(l) => l,
        DBEntry::Nil => {
            let l = VecDeque::new();
            
            entry.set_list(l);
            
            entry.get_list().unwrap()
        },
        _ => return RESPType::Error("wrong type".into())
    };

    l.push_front("v".into());


    // let entry = match db.get_list(key) {
    //     Err(DBError::WrongType) => return RESPType::Error("wrong type".into()),
    //     Ok(Some(v)) => v,
    //     Ok(None) => &VecDeque::new(),
    // };

    // let Ok(mut parsed_value) = str::parse::<i64>(&v) else {
    //     return RESPType::Error("not a valid integer".into());
    // };

    // parsed_value += 1;

    // db.set(key, parsed_value.to_string(), ExpiryFlag::KeepTTL, ExistenceFlag::None);

    RESPType::Null
}