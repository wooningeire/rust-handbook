use std::{
	sync::{mpsc, Arc, Mutex},
	thread
};


pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
	/// Constructs a ThreadPool with `size` threads.
	///
	/// # Panics
	///
	/// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers: Vec<Worker> = Vec::with_capacity(size);

		for index in 0..size {
			workers.push(Worker::new(index, Arc::clone(&receiver)));
		}

		ThreadPool {
			workers,
			sender: Some(sender),
		}
	}

	pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
		if size > 0 {
			return Err(
				PoolCreationError {
					message: String::from("Number of threads must be positive"),
				}
			)
		}

		Ok(ThreadPool::new(size))
	}

	pub fn execute<F>(&self, function: F)
	where
		F: FnOnce() + Send + 'static,
	{
		let job = Box::new(function);
		self.sender.as_ref().unwrap().send(job).unwrap();
	}
}


impl Drop for ThreadPool {
	fn drop(&mut self) {
		drop(self.sender.take());

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}


pub struct PoolCreationError {
	pub message: String,
}


struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		Worker {
			id,
			thread: Some(thread::spawn(move || loop {
				// Let's try to gain an intuition of what's going on here.
				// Here's a timeline:
				// 
				// M = got mutex, now waiting on receiver
				// R! = received request from mpsc, now running job
				// X = job finished, now waiting on mutex
				//
				// The next worker to reserve the mutex is the first to have
				// called `Mutex::lock` (X; here: left to right, top to bottom)
				//
				// time		worker
				// v		0		1		2
				// |		M		X		X
				// v		.
				// |		.
				// v		.
				// |		.
				// v		R!		M
				// |		.		.
				// v		X		.
				// |				R!		M
				// v				.		.
				// |		M		.		R!
				// v		.		.		.
				// |		.		.		X
				// v		.		.		
				// |		.		.		
				// v		.		.		
				// |		R!		.		M
				// v				X		.
				// |						.
				let received = receiver.lock().unwrap().recv();

				match received {
					Ok(job) => {
						println!("Worker {id} received a job!!!!!!!!!!");
						job();
					}
					Err(_) => {
						println!("Worker {id} disconnected..............");
						break;
					}
				}
			})),
		}
	}
}