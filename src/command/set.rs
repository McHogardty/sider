

use bytes::Bytes;
use chrono::{TimeZone, Utc, LocalResult, Duration};
use command_macro::command;

use sider_command::RESPType;
use crate::{db::{ExpiryFlag, ExistenceFlag, DBError, DBString, DBEntry}, util::from_decimal_bytes};

use super::super::db::DB;



#[command(
    name = "set",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn set(args: Vec<RESPType<Vec<u8>>>, db: &mut DB) -> RESPType<Bytes> {
    if args.len() < 2 {
        return RESPType::Error("wrong number of arguments".into());
    }

    let RESPType::BulkString(key) = &args[0] else {
        return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
    };

    let RESPType::BulkString(value) = &args[1] else {
        return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
    };

    let mut remaining = args[2..].iter().peekable();

    let mut expiry = ExpiryFlag::None;
    let mut existence_flag = ExistenceFlag::None;
    let mut return_previous_value = false;

    while let Some(current) = remaining.next() {
        let next = remaining.peek();

        let RESPType::BulkString(s) = current else {
            return RESPType::Error("Invalid command format, expecting array of bulk strings.".into());
        };
        b"123";

        match (&s[..], next) {
            (b"GET", _) if !return_previous_value => {
                return_previous_value = true;
            },
            (b"NX", _) if existence_flag.is_none() => {
                existence_flag = ExistenceFlag::Nx;
            },
            (b"XX", _) if existence_flag.is_none() => {
                existence_flag = ExistenceFlag::Xx;
            },
            (f @ (b"EX" | b"PX" | b"EXAT" | b"PXAT"), Some(RESPType::BulkString(e))) if expiry.is_none() => {
                let Ok(ex) = from_decimal_bytes(e) else {
                    return RESPType::Error("Invalid syntax.".into());
                };
                
                expiry = ExpiryFlag::Some(match f {
                    b"EX" => {
                        Utc::now() + Duration::seconds(ex)
                    },
                    b"PX" => {
                        Utc::now() + Duration::milliseconds(ex)
                    },
                    b"EXAT" => {
                        let LocalResult::Single(e) = Utc.timestamp_opt(ex, 0) else {
                            return RESPType::Error("Invalid syntax.".into());
                        };

                        e
                    },
                    b"PXAT" => {
                        let LocalResult::Single(e) = Utc.timestamp_millis_opt(ex) else {
                            return RESPType::Error("Invalid syntax.".into());
                        };

                        e
                    },
                    _ => unreachable!(),
                });
                
                remaining.next();
            },
            (b"KEEPTTL", _) if expiry.is_none() => {
                expiry = ExpiryFlag::KeepTTL;
            },
            _ => {
                return RESPType::Error("Invalid syntax.".into());
            }
        }
    }

    let entry = match db.get_or_insert(key.clone(), expiry, existence_flag) {
        Ok(e) => e,
        Err(DBError::AlreadyExists | DBError::DoesNotExist) => return RESPType::Null,
        Err(_) => panic!()
    };

    if !entry.is_string() && !entry.is_nil() {
        return RESPType::Error("wrong type".into())
    }

    let previous = entry.set_string(DBString::from_bytes(value.clone()));

    if return_previous_value {
        match previous {
            DBEntry::Nil => RESPType::Null,
            DBEntry::String(s) => RESPType::BulkString(s.to_bytes()),
            _ => unreachable!(),
        }
    } else {
        RESPType::SimpleString(Bytes::from("OK"))
    }
}