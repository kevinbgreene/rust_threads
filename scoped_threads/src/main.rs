use std::thread;
use std::time::Duration;

fn main() {
    let str = "What's that?";

    // Single immutable borrow: not okay.
    // let h = thread::spawn(|| {
    //     println!("str = {}", str);
    // });
    // h.join();

    let mut y = 0;
    thread::scope(|s| {
        // We can have multiple immutable borrows.
        s.spawn(|| {
            println!("str = {}", str);
        });

        s.spawn(|| {
            println!("str = {}", str);
        });
    
        // Or a single mutable borrow.
        s.spawn(|| {
            thread::sleep(Duration::from_millis(1000));
            y += 1;
        });
    });

    println!("y = {}", y);
}
