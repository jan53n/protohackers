pub mod pool;

use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use pool::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let mut payload: Vec<u8> = Vec::new();

    stream.read_to_end(&mut payload).expect("failed to read!");
    stream.write_all(&payload).expect("failed to write!");
}

fn main() -> std::io::Result<()> {
    let default_port: Option<String> = Some("8888".to_string());
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).and(default_port).unwrap();
    let addr = format!("0.0.0.0:{}", port);
    let pool = ThreadPool::new(5);
    let listener = TcpListener::bind(addr.as_str())?;

    println!("[info] using {}", &addr);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|| {
            handle_client(stream);
        });
    }
    Ok(())
}
