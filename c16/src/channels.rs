use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// pub fn run() {
// 	let (tx, rx) = mpsc::channel();

// 	thread::spawn(move || {
// 		let string = String::from("hi");
// 		tx.send(string).unwrap();
// 	});

// 	let received = rx.recv().unwrap();
// 	println!("Got: {}", received);
// }

pub fn run() {
	let (tx, rx) = mpsc::channel();

	let tx0 = tx.clone();
	thread::spawn(move || {
		let vals: Vec<String> = vec!["hello", "I", "am", "here"]
				.iter()
				.map(|string| String::from(*string))
				.collect();

		for val in vals {
			tx0.send(val).unwrap();
			thread::sleep(Duration::from_secs(1))
		}
	});

	
	thread::spawn(move || {
		let vals: Vec<String> = vec!["a", "b", "c"]
				.iter()
				.map(|string| String::from(*string))
				.collect();

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_millis(1500));
		}
	});


	for received in rx {
		println!("Got: {}", received)
	}
}