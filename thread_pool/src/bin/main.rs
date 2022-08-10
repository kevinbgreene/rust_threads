use std::{thread, time::Duration};

use rand::Rng;
use thread_pool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);

    for _ in 0..10 {
        pool.spawn(|| {
            thread::sleep(Duration::from_millis(1000));
            println!("Work in thread = {}", thread::current().name().unwrap());
        });

        let delay = rand::thread_rng().gen_range(10..=2000);
        thread::sleep(Duration::from_millis(delay));
    }

    let mut handles = Vec::with_capacity(10);

    for num in 0..10 {
        handles.push(pool.spawn(move || {
            let delay = rand::thread_rng().gen_range(10..=1000);
            thread::sleep(Duration::from_millis(delay));
            num
        }));
    }

    for handle in handles {
        println!("result = {}", handle.join().unwrap());
    }
}
