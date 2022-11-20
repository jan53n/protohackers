use std::{collections::HashMap, net::UdpSocket, str::FromStr};

enum Request {
    Insert(String, String),
    Retrieve(String),
}

fn parse_request(req: &[u8]) -> Result<Request, ()> {
    let payload = String::from_utf8(req.to_vec()).unwrap();

    if let Some(eq_loc) = payload.find('=') {
        return Ok(Request::Insert(
            String::from_str(&payload[0..eq_loc]).unwrap(),
            String::from_str(&payload[eq_loc..]).unwrap(),
        ));
    } else {
        return Ok(Request::Retrieve(payload));
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8888")?;
    let mut db: HashMap<String, String> = HashMap::default();

    loop {
        let mut buf = [0; 512];
        let (_, src) = socket.recv_from(&mut buf)?;

        if let Ok(req) = parse_request(&buf) {
            match req {
                Request::Insert(key, value) => {
                    db.insert(key, value).unwrap();
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
