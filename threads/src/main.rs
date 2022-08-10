use rand::{self, Rng};
use std::{thread, thread::JoinHandle, time::Duration};

fn single_spawn() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        println!("Hello from spawned thread");
    });

    handle.join().unwrap();
}

fn multiple_spawn() {
    use std::time::Instant;
    let now = Instant::now();

    let handles: Vec<JoinHandle<String>> = (0..=10)
        .map(|i| {
            let delay = rand::thread_rng().gen_range(10..=2000);
            let builder = thread::Builder::new().name(format!("Thread-{}", i));

            builder
                .spawn(move || {
                    // println!("thread started = {}", thread::current().name().unwrap());
                    thread::sleep(Duration::from_millis(delay));
                    thread::current().name().unwrap().to_owned()
                })
                .unwrap()
        })
        .collect();

    for h in handles {
        let r = h.join().unwrap();
        // println!("thread done = {:?}", r);
    }

    let elapsed = now.elapsed();
    println!("elapsed = {:.2?}", elapsed);
}

fn fork_join() {
    let first_name_handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        "Kevin"
    });

    let last_name_handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        "Greene"
    });

    let name = format!(
        "{} {}",
        first_name_handle.join().unwrap(),
        last_name_handle.join().unwrap()
    );

    println!("name = {}", name);
}

fn main() {
    single_spawn();
    multiple_spawn();
    fork_join();
}
