pub mod message;
pub mod pool;

use crate::{
    message::{Message, RawMessage},
    pool::ThreadPool,
};

use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn get_mean_from_minmax(store: &Vec<i32>, min_time: i32, max_time: i32) -> i32 {
    let mut c = 0;
    let mut t = 0;

    for i in min_time..max_time {
        if let Some(v) = store.get(i as usize) {
            c += 1;
            t += v;
        }
    }

    t / c
}

fn handle_client(mut stream: TcpStream, _id: usize) -> Result<(), Box<dyn Error>> {
    let mut store = Vec::new();

    loop {
        let mut buf: RawMessage = [0; 9];
        stream.read_exact(&mut buf[..])?;

        println!("{:?}", buf);

        match Message::try_from(buf).unwrap_or(Message::Undefined) {
            Message::Insert { timestamp, price } => {
                store.insert(timestamp as usize, price);
            }
            Message::Query { min_time, max_time } => {
                let mean: i32 = get_mean_from_minmax(&store, min_time, max_time);
                stream.write_all(&mean.to_be_bytes()).unwrap();
            }
            Message::Undefined => {
                return Ok(());
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("please provide a valid port");
    let addr = format!("[::]:{}", port);
    let pool = ThreadPool::new(5);
    let listener = TcpListener::bind(addr.as_str())?;

    println!("[info] using {}", &addr);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|id| {
            handle_client(stream, id).unwrap();
        });
    }
    Ok(())
}
