use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Task = dyn FnOnce() + Send + 'static;

type BoxedTask = Box<Task>;

type SharedReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

enum Message {
    RunTask(BoxedTask),
    Terminate,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let mut threads = Vec::with_capacity(num_threads);
        let (sender, receiver) = mpsc::channel();
        let receiver: SharedReceiver = Arc::new(Mutex::new(receiver));

        for id in 0..num_threads {
            let receiver = Arc::clone(&receiver);
            let worker = Worker::new(id, receiver);
            threads.push(worker);
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::RunTask(Box::new(f))).unwrap();
    }

    pub fn spawn<F, T>(&self, f: F) -> TaskHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (sender, receiver) = mpsc::sync_channel::<T>(0);
        let task = move || {
            sender.send(f()).unwrap();
        };

        self.sender.send(Message::RunTask(Box::new(task))).unwrap();

        TaskHandle::new(receiver)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }
    }
}

pub struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: SharedReceiver) -> Self {
        let builder = thread::Builder::new().name(format!("worker-thread-{}", id));
        let thread = builder
            .spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::RunTask(task) => task(),
                    Message::Terminate => {
                        break;
                    }
                }
            })
            .unwrap();

        Worker { thread }
    }
}

pub struct TaskHandle<T> {
    receiver: mpsc::Receiver<T>
}

impl<T> TaskHandle<T> {
    pub fn new(receiver: mpsc::Receiver<T>) -> Self {
        TaskHandle { receiver }
    }

    pub fn join(&self) -> Result<T, ()> {
        Ok(self.receiver.recv().unwrap())
    }
}
