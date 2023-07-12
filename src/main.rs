#![feature(const_mut_refs)]

mod db;
mod parser;
mod serializer;
mod server;
mod command;
mod util;

use env_logger;

use crate::server::Server;

fn main() {
    env_logger::init();

    match Server::build().start() {
        Ok(()) => (),
        Err(e) => {
            println!("Server encountered an error: {:?}", e)
        }
    }
}
