pub mod pool;
pub mod room;

use room::{send, Room};

use crate::{
    pool::ThreadPool,
    room::{Event, EventType::*},
};
use std::{
    env,
    error::Error,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
};

static ASKING_FOR_NAME: &str = "Welcome to budgetchat! What shall I call you?\n";
static INVALID_USERNAME: &str = "Invalid username, bye!\n";

fn eat_username(stream: &mut BufReader<TcpStream>) -> Result<String, String> {
    let mut buf = String::new();

    if stream.read_line(&mut buf).is_ok() && buf.len() >= 4 && buf.len() <= 20 {
        return Ok(buf[0..buf.len() - 1].to_string());
    }

    Err(String::from(INVALID_USERNAME))
}

fn handle_client(
    mut stream: TcpStream,
    room: &Arc<Mutex<Sender<Event>>>,
) -> Result<(), Box<dyn Error>> {
    let addr = stream.peer_addr().unwrap();
    let mut reader = BufReader::new(stream.try_clone()?);

    stream.write_all(ASKING_FOR_NAME.as_bytes())?;

    match eat_username(&mut reader) {
        Ok(username) => send(
            &room,
            Event::new(NewUser(username, stream.try_clone().unwrap()), addr),
        ),
        Err(_error) => {
            stream.write_all(_error.as_bytes())?;
            return Ok(());
        }
    };

    for line in reader.lines() {
        send(&room, Event::new(Broadcast(line?), addr));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).expect("please provide a valid port");
    let addr = format!("[::]:{}", port);
    let pool = ThreadPool::new(11);
    let listener = TcpListener::bind(addr.as_str())?;
    let (room_sender, room_receiver) = channel::<Event>();
    let room_sender = Arc::new(Mutex::new(room_sender));

    // room
    pool.execute(|_| {
        let mut room = Room::default();
        room.listen_for_events(room_receiver);
    });

    for stream in listener.incoming() {
        let stream = stream?;
        let peer_addr = stream.peer_addr().unwrap();
        let r_s = Arc::clone(&room_sender);

        pool.execute(move |_| {
            if let Err(e) = handle_client(stream, &r_s) {
                println!("{:?}", e);
            }

            send(&r_s, Event::new(Disconnect, peer_addr));
        });
    }
    Ok(())
}
