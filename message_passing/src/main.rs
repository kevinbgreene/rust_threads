use rand::{self, Rng};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn example_one() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val: i32 = receiver.recv().unwrap();
        val + 5
    });

    sender.send(8).unwrap();

    println!("result = {}", handle.join().unwrap());
}

fn example_two() {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val: i32 = receiver.recv().unwrap();
        val + 5
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(2000));
        sender.send(8).unwrap();
    });

    println!("result = {}", handle.join().unwrap());
}

fn channel_iterator_loop() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for val in receiver {
            println!("val = {}", val);
        }
    });

    for i in 0..10 {
        sender.send(i).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}

fn channel_iterator_manual() {
    let (sender, receiver) = mpsc::channel();
    for i in 0..10 {
        sender.send(i).unwrap();
    }
    let mut iter = receiver.iter();
    println!("next = {:?}", iter.next());
    println!("next = {:?}", iter.next());
}

fn shared_receiver() {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..4 {
        let receiver = Arc::clone(&receiver);
        thread::spawn(move || loop {
            let val = receiver.lock().unwrap().recv().unwrap();
            println!("val = {}, from thread-{}", val, id);
            let delay = rand::thread_rng().gen_range(100..=500);
            thread::sleep(Duration::from_millis(delay));
        });
    }

    for i in 0..100 {
        sender.send(i).unwrap();
        let delay = rand::thread_rng().gen_range(1..=1000);
        thread::sleep(Duration::from_millis(delay));
    }
}

fn returning_values() {
    fn spawn_thread() -> Receiver<i32> {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            sender.send(5).unwrap();
        });

        receiver
    }

    let receiver = spawn_thread();
    let val = receiver.recv().unwrap();
    println!("val = {}", val);
}

fn main() {
    // example_one();
    // example_two();
    // channel_iterator_loop();
    // channel_iterator_manual();
    // shared_receiver();
    returning_values();
}
