mod message;
mod pool;

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use message::rewrite_message_with_coin;
use pool::ThreadPool;

const UPSTREAM_SERVER: &str = "chat.protohackers.com:16963";
const LISTEN_ADDR: &str = "[::]:12345";

fn read_write(rs: TcpStream, mut ws: TcpStream) {
    let mut reader = BufReader::new(rs.try_clone().unwrap());

    loop {
        let mut buf = String::new();

        let len = reader.read_line(&mut buf).unwrap();

        if len == 0 {
            break;
        }

        println!("{buf:?}");

        ws.write_all(rewrite_message_with_coin(buf).as_bytes())
            .unwrap();
    }
}

fn handle_client(stream: TcpStream) {
    let server = TcpStream::connect(UPSTREAM_SERVER).unwrap();

    let (client_reader, client_writer, server_writer, server_reader) = (
        stream.try_clone().unwrap(),
        stream.try_clone().unwrap(),
        server.try_clone().unwrap(),
        server.try_clone().unwrap(),
    );

    thread::spawn(move || {
        read_write(client_reader, server_writer);
    });

    thread::spawn(move || {
        read_write(server_reader, client_writer);
    });
}

fn main() {
    let listener = TcpListener::bind(LISTEN_ADDR).unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|_| {
            handle_client(stream);
        });
    }
}
