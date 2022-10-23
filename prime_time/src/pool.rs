use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce(usize) + Send + 'static>;

pub struct ThreadPool {
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            Worker::init(id, Arc::clone(&receiver));
        }

        Self { sender }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce(usize) + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(job).expect("failed to send message!");
    }
}

pub struct Worker {}

impl Worker {
    pub fn init(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) {
        thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("[info] job processing on #{}", id);
                    job(id);
                }
                _ => {
                    println!("[error] shutting down worker #{}", id);
                    break;
                }
            }

            println!("[info] job completed #{}", id);
        });
    }
}
