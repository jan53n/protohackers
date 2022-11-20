use std::{collections::HashMap, net::UdpSocket};

#[derive(Debug)]
enum Request {
    Insert(String, String),
    Retrieve(String),
}

fn parse_request(req: &[u8]) -> Result<Request, ()> {
    let payload = core::str::from_utf8(req).unwrap();

    if let Some(eq_loc) = payload.find('=') {
        Ok(Request::Insert(
            payload[0..eq_loc].to_string(),
            payload[eq_loc + 1..].to_string(),
        ))
    } else {
        Ok(Request::Retrieve(payload.to_string()))
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8888")?;
    let mut db: HashMap<String, String> = HashMap::default();

    loop {
        let mut buf = [0; 512];
        let (_, src) = socket.recv_from(&mut buf)?;

        if let Ok(req) = parse_request(&buf) {
            println!("{:?}", req);

            match req {
                Request::Insert(key, value) => {
                    db.entry(key)
                        .and_modify(|v| *v = value.clone())
                        .or_insert(value);
                }
                Request::Retrieve(key) => {
                    if key == "version" {
                        socket
                            .send_to("version=Ken's Key-Value Store 1.0".as_bytes(), src)
                            .unwrap();
                    } else if let Some(value) = db.get(&key) {
                        socket.send_to(value.as_bytes(), src).unwrap();
                    } else {
                        socket.send_to("".as_bytes(), src).unwrap();
                    }
                }
            }
        }
    }
}
