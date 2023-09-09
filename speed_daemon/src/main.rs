use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("[::]:5000")?;

    fn handle_client(mut stream: TcpStream) {
        let mut buf = [0];
        stream.read_exact(&mut buf).unwrap();
        println!("{:?}", buf[0]);
        stream.write(&[0x10]).unwrap();
    }

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream);
    }

    Ok(())
}
