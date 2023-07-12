
use command_macro::command;

use sider_command::RESPType;
use super::{super::db::DB, responses};


#[command(
    name = "ping",
    arity = -1,
    flags = ("fast", "sentinel"),
    first_key = 1,
    last_key = 2,
    step = 1,
    acl_categories = ("connection"),
    command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
)]
pub fn ping(_args: Vec<RESPType>, _: &mut DB) -> RESPType {
    RESPType::SimpleString(responses::PONG.into())
}