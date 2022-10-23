pub mod pool;

use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use is_prime::is_prime as is_p;
use num_bigint::BigInt;
use serde::{ser::Error, Deserialize, Serialize};

use pool::ThreadPool;
use serde_json::{Number, Result};

static MALFORMED_RESPONSE: &[u8; 2] = b"{\n";

#[derive(Deserialize, Debug)]
struct Request {
    method: String,
    number: Number,
}

#[derive(Serialize, Debug)]
struct Response {
    method: String,
    prime: bool,
}

fn is_valid_request(req: &Request) -> bool {
    req.method == "isPrime"
}

fn is_prime(n: &Number) -> bool {
    let big = BigInt::from_str(n.to_string().as_str()).unwrap_or(BigInt::from(0));

    if big <= BigInt::from(1) {
        return false;
    }

    is_p(big.to_string().as_str())
}

fn handle_request(req: &String) -> Result<String> {
    let req: Request = serde_json::from_str(&req)?;

    if !is_valid_request(&req) {
        return Err(Error::custom("invalid request"));
    }

    let response = Response {
        method: "isPrime".to_string(),
        prime: is_prime(&req.number),
    };

    let mut string_response = serde_json::to_string(&response).unwrap();
    string_response.push_str("\n");

    Ok(string_response)
}

fn handle_client(mut stream: TcpStream, id: usize) {
    let stream_cloned = stream.try_clone().unwrap();
    let reader = BufReader::new(stream_cloned);

    for raw_request in reader.lines() {
        let req = raw_request.expect("[error] unexpected line!");
        let response = handle_request(&req);

        match &response {
            Err(_) => {
                stream.write_all(MALFORMED_RESPONSE).unwrap();
                println!("[debug] {:?} -> {:?}", &req, MALFORMED_RESPONSE);
                break;
            }
            Ok(r) => {
                if stream.write_all(r.as_bytes()).is_err() {
                    break;
                }
            }
        }

        println!("[debug] #{} {:?} -> {:?}", id, &req, &response);
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
