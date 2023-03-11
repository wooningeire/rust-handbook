use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run() {
	let counter1 = Arc::new(Mutex::new(0));
	let counter2 = Arc::new(Mutex::new(0));


	let a1_1 = Arc::clone(&counter1);
	let a2_1 = Arc::clone(&counter2);
	let handle1 = thread::spawn(move || {
		println!("Thread 1: waiting for counter1!");
		let mut n = a1_1.lock().unwrap();
		println!("Thread 1: locked counter1!");
		thread::sleep(Duration::from_secs(1));
		
		println!("Thread 1: waiting for counter2!");
		let m = a2_1.lock().unwrap();
		println!("Thread 1: locked counter2!");

		*n += *m + 1;
	});


	let a1_2 = Arc::clone(&counter1);
	let a2_2 = Arc::clone(&counter2);
	let handle2 = thread::spawn(move || {
		println!("Thread 2: waiting for counter2!");
		let mut n = a2_2.lock().unwrap();
		println!("Thread 2: locked counter2!");
		thread::sleep(Duration::from_secs(1));

		println!("Thread 2: waiting for counter1!");
		let m = a1_2.lock().unwrap();
		println!("Thread 2: locked counter1!");

		*n += *m + 1;
	});

	
	handle1.join().unwrap();
	handle2.join().unwrap();

	println!("Finished @ counter1 = {}, counter2 = {}",
			*counter1.lock().unwrap(),
			*counter2.lock().unwrap());
}