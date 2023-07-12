
use phf::Map;
use phf_macros::phf_map;

mod base;
mod responses;

mod command;
mod decr;
mod del;
mod echo;
mod exists;
mod get;
mod incr;
mod lpush;
mod ping;
mod set;


pub(crate) const COMMAND_TABLE: Map<&'static [u8], base::Command> = phf_map! {
    b"command" => command::CommandImpl::into_command(),
    b"decr" => decr::Decr::into_command(),
    b"del" => del::Del::into_command(),
    b"echo" => echo::Echo::into_command(),
    b"exists" => exists::Exists::into_command(),
    b"get" => get::Get::into_command(),
    b"incr" => incr::Incr::into_command(),
    b"lpush" => lpush::Lpush::into_command(),
    b"ping" => ping::Ping::into_command(),
    b"set" => set::Set::into_command(),
};