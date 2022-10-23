pub mod pool;

use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use serde::{ser::Error, Deserialize, Serialize};

use pool::ThreadPool;
use serde_json::Result;

static MALFORMED_RESPONSE: &[u8; 2] = b"{\n";

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

fn handle_request(req: &String) -> Result<String> {
    let req: Request = serde_json::from_str(&req)?;

    if !is_valid_request(&req) {
        return Err(Error::custom("invalid request"));
    }

    let response = Response {
        method: "isPrime".to_string(),
        prime: is_prime(req.number),
    };

    let mut string_response = serde_json::to_string(&response).unwrap();
    string_response.push_str("\n");

    Ok(string_response)
}

fn handle_client(mut stream: TcpStream, id: usize) {
    let stream_cloned = stream.try_clone().unwrap();
    let reader = BufReader::new(stream_cloned);

    for raw_request in reader.lines() {
        let req = raw_request.unwrap();
        let rr = req.clone();
        let response = handle_request(&req);

        println!("[debug] #{} {:?}, {:?}", id, &req, &response);

        match &response {
            Err(_) => {
                stream.write_all(MALFORMED_RESPONSE).unwrap();
                break;
            }
            Ok(r) => {
                if stream.write_all(r.as_bytes()).is_err() {
                    break;
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("please provide a port");
    let addr = format!("[::]:{}", port);
    let pool = ThreadPool::new(5);
    let listener = TcpListener::bind(addr.as_str())?;

    println!("[info] using {}", &addr);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|id| {
            handle_client(stream, id);
        });
    }
    Ok(())
}
