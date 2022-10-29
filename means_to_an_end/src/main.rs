pub mod message;
pub mod pool;
pub mod store;

use crate::{
    message::{Message, RawMessage},
    pool::ThreadPool,
    store::get_mean_from_minmax_time,
};
use std::{
    env,
    error::Error,
    io::{Read, Write},
    net::TcpListener,
};
use store::PriceStore;

fn handle_client(mut stream: impl Read + Write, _id: usize) -> Result<(), Box<dyn Error>> {
    let mut store = PriceStore::new();

    loop {
        let mut buf: RawMessage = [0; 9];

        stream.read_exact(&mut buf[..])?;

        let message = Message::try_from(buf).unwrap_or(Message::Undefined);

        match message {
            Message::Insert { timestamp, price } => {
                store.insert(timestamp, price);
            }
            Message::Query { min_time, max_time } => {
                let mean: i32 = get_mean_from_minmax_time(&store, min_time, max_time);
                println!("{} - {} -> {}", min_time, max_time, mean);
                stream.write_all(&mean.to_be_bytes()).unwrap();
            }
            Message::Undefined => {}
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
            if let Err(e) = handle_client(stream, id) {
                println!("{:?}", e);
            }
        });
    }
    Ok(())
}
