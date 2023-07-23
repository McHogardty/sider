

use std::collections::VecDeque;
use std::error::Error;
use std::io::{BufWriter, Read, ErrorKind, Write};
use std::time::{Duration, Instant};

use bytes::{Bytes, BytesMut};
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
    output_buffer: BufWriter<TcpStream>,
}


fn handle_command(db: &mut DB, command: RESPType<Bytes>) -> RESPType<Bytes> {
    let RESPType::Array(mut v) = command else {
        return RESPType::Error(Bytes::from("Invalid command format, expecting array of bulk strings."));
    };

    if v.len() == 0 {
        return RESPType::Error(Bytes::from("Invalid command format, array must have at least one element."));
    }

    let command_name = v.remove(0);

    let RESPType::BulkString(s) = command_name else {
        return RESPType::Error(Bytes::from("Invalid command format, expecting array of bulk strings."));
    };

    let s = s.to_ascii_lowercase();

    let Some(command) = COMMAND_TABLE.get(&s) else {
        return RESPType::Error(Bytes::from("Invalid command."));
    };

    return (command.handler)(v, db);
}



pub struct Server {
    clients: Vec<Option<Client>>,
}


impl Server {
    pub fn build() -> Self {
        Server {
            clients: Vec::with_capacity(100),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut db = DB::new();
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);
        let mut client_request_queue: VecDeque<(Token, RESPType<Bytes>)> = VecDeque::new();

        let mut listener = TcpListener::bind("127.0.0.1:6379".parse().unwrap())?;
        let client_request_waker = Waker::new(poll.registry(), CLIENT_REQUEST_QUEUE)?;
        
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
                        
                        poll.registry().register(&mut connection, Token(self.clients.len() + 2), Interest::READABLE | Interest::WRITABLE)?;

                        self.clients.push(Some(Client {
                            output_buffer: BufWriter::new(connection),
                        }));

                    },
                    CLIENT_REQUEST_QUEUE => {
                        while let Some((client_token, request)) = client_request_queue.pop_front() {    
                            let response = match request {
                                RESPType::Error(e) => RESPType::Error(Bytes::from(e)),
                                r => handle_command(&mut db, r)
                            };

                            let output = match &mut self.clients[client_token.0 - 2] {
                                Some(client) => &mut client.output_buffer,
                                None => continue,
                            };
        
                            serialize(&response, output)?;
                            output.flush()?;

                        }
                    },
                    token => {
                        let client = match &mut self.clients[token.0 - 2] {
                            None => {
                                continue;
                            },
                            Some(client) => client,
                        };

                        if event.is_readable() {
                            let mut query_buffer = BytesMut::zeroed(1024*16);

                            let num_bytes_read = match client.output_buffer.get_mut().read(&mut query_buffer) {
                                Ok(n) => n,
                                Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                                Err(e) => return Err(e.into()),
                            };

                            if num_bytes_read == 0 {
                                poll.registry().deregister(client.output_buffer.get_mut())?;
                                self.clients[token.0 - 2] = None;
                                continue;
                            }

                            let result = RESPParser::parse(query_buffer.into());

                            client_request_queue.push_back((token, result));
                            client_request_waker.wake()?;
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

