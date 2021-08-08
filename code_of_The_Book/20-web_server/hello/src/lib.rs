use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
    // receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    TERMINATE,
}

impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn( move || loop {
             
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::TERMINATE => {
                    println!("Worker {} is terminated.", id);
                    break;
                }
            }

        });

        Worker {id, thread: Some(thread)}
    }
}

impl ThreadPool {

    pub fn new (n: u32) -> ThreadPool {
        assert!(n > 0);

        let mut workers = Vec::new();

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..n {
            let receiver = Arc::clone(&receiver);
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool{workers, sender}
    }

    pub fn execute<F:FnOnce() + Send + 'static>(&self, f: F)
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Message::TERMINATE);
        }

        for worker in &mut self.workers {
            //println!("Shutting down worker {}.", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}