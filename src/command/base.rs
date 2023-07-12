
use sider_command::*;
use super::super::db::DB;


pub struct Command<'a> {
    pub name: &'a str,
    pub handler: fn(Vec<RESPType>, &mut DB) -> RESPType,
    pub arity: i64,
    pub flags: &'a [Flag],
    pub first_key: u64,
    pub last_key: i64,
    pub step: u64,
    pub acl_categories: &'a [AclCategory],
    pub tips: &'a [CommandTip],
}
