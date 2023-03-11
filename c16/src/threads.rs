use std::thread;
use std::time::Duration;

pub fn run() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hello number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        println!("Ha! It's my vector now: {:?}", v);
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hello number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))
    }

    // println!("Oops, {:?}", v);
}