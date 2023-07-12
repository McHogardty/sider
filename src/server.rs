
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{Read, ErrorKind};
use std::time::{Duration, Instant};

use mio::{Poll, Events, Token, Interest, Waker};
use mio::net::{TcpListener, TcpStream};

use sider_command::RESPType;
use crate::command::COMMAND_TABLE;
use crate::db::DB;
use crate::parser::RESPParser;
use crate::serializer::serialize;



const SERVER: Token = Token(0);
const CLIENT_REQUEST_QUEUE: Token = Token(1);


#[derive(Debug)]
struct Client {
    connection: TcpStream,
    query_buffer: [u8; 1024*16],
}


fn handle_command(db: &mut DB, command: RESPType) -> RESPType {
    let RESPType::Array(mut v) = command else {
        return RESPType::Error(String::from("Invalid command format, expecting array of bulk strings."));
    };

    if v.len() == 0 {
        return RESPType::Error(String::from("Invalid command format, array must have at least one element."));
    }

    let command_name = v.remove(0);

    let RESPType::BulkString(mut s) = command_name else {
        return RESPType::Error(String::from("Invalid command format, expecting array of bulk strings."));
    };

    s.make_ascii_lowercase();

    let Some(command) = COMMAND_TABLE.get(&s) else {
        return RESPType::Error(String::from("Invalid command."));
    };

    return (command.handler)(v, db);
}



pub struct Server {
    clients: HashMap<Token, Client>,
}


impl Server {
    pub fn build() -> Self {
        Server {
            clients: HashMap::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut db = DB::new();
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);
        let mut client_request_queue: VecDeque<(Token, RESPType)> = VecDeque::new();

        let mut listener = TcpListener::bind("127.0.0.1:6379".parse().unwrap())?;
        let client_request_waker = Waker::new(poll.registry(), CLIENT_REQUEST_QUEUE)?;

        let mut next_writable_token = Token(2);
        
        poll.registry().register(&mut listener, SERVER, Interest::READABLE).unwrap();
        
        let background_task_frequency = Duration::from_millis(100);

        loop {
            let next_background_task = Instant::now() + background_task_frequency;

            poll.poll(&mut events, Some(background_task_frequency))?;

            for event in &mut events.iter() {
                match event.token() {
                    SERVER => loop {
                        let (mut connection, _) = match listener.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) => {
                                if e.kind() == ErrorKind::WouldBlock {
                                    break;
                                }

                                return Err(e.into());
                            }
                        };
                        poll.registry().register(&mut connection, next_writable_token, Interest::READABLE | Interest::WRITABLE)?;
                        
                        self.clients.insert(next_writable_token, Client {
                            connection,
                            query_buffer: [0; 16384],
                        });

                        next_writable_token.0 += 1;
                    },
                    CLIENT_REQUEST_QUEUE => {
                        while let Some((client_token, request)) = client_request_queue.pop_front() {    
                        }
                    },
                    token => {
                        let client = match self.clients.get_mut(&token) {
                            None => {
                                continue;
                            },
                            Some(client) => client,
                        };

                        if event.is_readable() {
                            let num_bytes_read = match client.connection.read(&mut client.query_buffer) {
                                Ok(n) => n,
                                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                                Err(e) => return Err(e.into()),
                            };

                            if num_bytes_read == 0 {
                                poll.registry().deregister(&mut client.connection)?;
                                continue;
                            }

                            let result = RESPParser::parse(&client.query_buffer);

                            let response = match result {
                                RESPType::Error(_) => result,
                                r => handle_command(&mut db, r)
                            };
        
                            serialize(&response, &mut client.connection)?;


                            // client_request_queue.push_back((token, result));
                            // client_request_waker.wake()?;
                        }
                    }
                }
            }

            if next_background_task < Instant::now() {
                db.expire_keys();
            }
        }
    }
}

