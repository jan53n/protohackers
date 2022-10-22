pub mod pool;

use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

use serde::{ser::Error, Deserialize, Serialize};

use pool::ThreadPool;
use serde_json::Result;

#[derive(Deserialize, Debug)]
struct Request {
    method: String,
    number: i32,
}

#[derive(Serialize, Debug)]
struct Response {
    method: String,
    prime: bool,
}

fn is_valid_request(req: &Request) -> bool {
    req.method == "isPrime"
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..(n - 1) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn handle_request(req: &String) -> Result<Response> {
    let req: Request = serde_json::from_str(&req)?;

    if !is_valid_request(&req) {
        return Err(Error::custom("invalid request"));
    }

    Ok(Response {
        method: "isPrime".to_string(),
        prime: is_prime(req.number),
    })
}

fn handle_client(stream: TcpStream) {
    let mut s = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    for raw_request in reader.lines() {
        let response = handle_request(&raw_request.unwrap());

        match response {
            Err(_) => {
                s.write_all("{\n".as_bytes()).unwrap();
                return;
            }
            Ok(r) => {
                let mut res = serde_json::to_string(&r).unwrap();
                res.push_str("\n");
                s.write_all(res.as_bytes()).unwrap();
            }
        }
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
