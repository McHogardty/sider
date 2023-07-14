
use command_macro::command;

use sider_command::RESPType;

use super::COMMAND_TABLE;
use super::super::db::DB;


// #[command(
//     name = "command",
//     arity = -1,
//     flags = ("fast", "sentinel"),
//     first_key = 1,
//     last_key = 2,
//     step = 1,
//     acl_categories = ("connection"),
//     command_tips = ("request_policy:all_shards", "response_policy:all_succeeded"),
// )]
// pub fn command_impl(_args: Vec<RESPType<Vec<u8>>>, _: &mut DB) -> RESPType<&[u8]> {
//     RESPType::Array(
//         COMMAND_TABLE.values().map(|v|
//             RESPType::Array(vec![
//                 RESPType::SimpleString(v.name.bytes().collect()),
//                 RESPType::Integer(v.arity),
//                 RESPType::Array(v.flags.iter().map(|f| RESPType::SimpleString(f.to_string().bytes().collect())).collect()),
//                 RESPType::Integer(v.first_key.try_into().unwrap()),
//                 RESPType::Integer(v.last_key),
//                 RESPType::Integer(v.step.try_into().unwrap()),
//                 RESPType::Array(v.acl_categories.iter().map(|c| RESPType::SimpleString(c.to_string().bytes().collect())).collect()),
//                 RESPType::Array(vec![]),
//                 RESPType::Array(vec![]),
//                 RESPType::Array(vec![]),
//             ])
//         ).collect()
//     )
// }