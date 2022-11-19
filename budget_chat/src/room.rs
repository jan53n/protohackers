use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

#[derive(Debug)]
pub enum EventType {
    NewUser(String, TcpStream),
    Disconnect,
    Broadcast(String),
}

#[derive(Debug)]
pub struct Event {
    pub etype: EventType,
    pub address: SocketAddr,
}

impl Event {
    pub fn new(etype: EventType, address: SocketAddr) -> Self {
        Self { etype, address }
    }
}

pub struct Connection {
    username: String,
    stream: RefCell<TcpStream>,
}

impl Connection {
    pub fn new(username: String, stream: TcpStream) -> Self {
        Self {
            username,
            stream: RefCell::new(stream),
        }
    }

    pub fn get_stream(&self) -> RefMut<TcpStream> {
        return self.stream.borrow_mut();
    }
}

#[derive(Default)]
pub struct Room {
    connections: HashMap<SocketAddr, Connection>,
}

impl Room {
    pub fn push(&mut self, connection: Connection) {
        let addr = connection.get_stream().peer_addr().unwrap();
        self.connections.insert(addr, connection);
    }

    pub fn remove(&mut self, addr: SocketAddr) {
        self.connections.remove(&addr);
    }

    pub fn broadcast(&mut self, message: &[u8], except: SocketAddr) {
        for (addr, con) in self.connections.iter() {
            let mut s = con.get_stream();

            if except == *addr {
                continue;
            }

            if let Err(_) = s.write_all(message) {
                println!("failed to write to {}", addr)
            }
        }
    }

    pub fn listen_for_events(&mut self, reciever: Receiver<Event>) {
        loop {
            if let Ok(event) = reciever.recv() {
                match event.etype {
                    EventType::NewUser(username, stream) => {
                        self._on_new_user(username, stream);
                    }
                    EventType::Broadcast(message) => {
                        self._on_broadcast(message, event.address);
                    }
                    EventType::Disconnect => {
                        self._on_user_disconnect(event.address);
                    }
                }
            }
        }
    }

    fn _on_broadcast(&mut self, message: String, addr: SocketAddr) {
        if let Some(c) = self.connections.get(&addr) {
            self.broadcast(format!("[{}] {}\n", c.username, message).as_bytes(), addr);
        }
    }

    fn _on_new_user(&mut self, un: String, stream: TcpStream) {
        let mut stream = stream;
        let connection = Connection::new(un.clone(), stream.try_clone().unwrap());

        self.push(connection);

        // broadcast new join
        self.broadcast(
            format!("* {} has entered the room\n", un).as_bytes(),
            stream.peer_addr().unwrap(),
        );

        // send list of users to the new joinee
        stream
            .write_all(
                format!("* The room contains: {}\n", self._list_of_usernames(&un)).as_bytes(),
            )
            .unwrap();
    }

    fn _on_user_disconnect(&mut self, addr: SocketAddr) {
        if let Some(c) = self.connections.get(&addr) {
            self.broadcast(
                format!("* {} has left the room\n", c.username).as_bytes(),
                addr,
            );

            self.connections.remove(&addr);
        }
    }

    fn _list_of_usernames(&self, exclude: &String) -> String {
        self.connections
            .values()
            .filter(|c| &c.username != exclude)
            .map(|c| c.username.clone())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

pub fn send(sender: &Arc<Mutex<Sender<Event>>>, event: Event) {
    sender.lock().unwrap().send(event).unwrap();
}
