// use std::mem::drop;

pub fn run() {
	let cp = CustomSmartPointer {
		data: String::from("my stuff"),
	};
	let dp = CustomSmartPointer {
		data: String::from("other stuff"),
	};
	println!("Pointers created");

	drop(dp);
}

struct CustomSmartPointer {
	data: String,	
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping pointer with data \"{}\"", self.data);	
	}
}