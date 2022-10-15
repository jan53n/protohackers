pub mod pool;

use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use serde::{Deserialize, Serialize};

use pool::ThreadPool;

#[derive(Deserialize)]
struct Request {
    method: String,
    number: i32,
}

#[derive(Serialize)]
struct Response {
    method: String,
    prime: bool,
}

fn is_valid_request(req: &Request) -> bool {
    req.method == "isPrime"
}

fn is_prime(_v: i32) -> bool {
    true
}

fn handle_request(req: &Request) -> Result<Response, &str> {
    Ok(Response {
        method: "isPrime".to_string(),
        prime: is_prime(req.number),
    })
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut reader = BufReader::new(&stream);
        let mut raw_json: Vec<u8> = Vec::new();

        reader
            .read_until(0xA, &mut raw_json)
            .expect("failed to read!");

        let request: Request = serde_json::from_slice(&raw_json[..]).expect("Invalid request!");

        if !is_valid_request(&request) {
            break;
        }

        let prime_response = handle_request(&request).unwrap();
        let response: Vec<u8> = serde_json::to_vec(&prime_response).unwrap();

        stream.write_all(&response).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("please provide a port");
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