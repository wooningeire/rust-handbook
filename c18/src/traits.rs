use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
	type Output = Millimeters;

	fn add(self, other: Meters) -> Millimeters {
		Millimeters(self.0 + other.0 * 1000)
	}
}


trait AmbiguousTrait {
	fn coolify(&self) -> Meters; 
}

trait UnpredictableTrait {
	fn coolify(&self) -> Meters;
}


impl AmbiguousTrait for Meters {
	fn coolify(&self) -> Meters {
		Meters(self.0 * self.0)
	}
}

impl UnpredictableTrait for Meters {
	fn coolify(&self) -> Meters {
		Meters(self.0 * 2)
	}
}

pub fn run() {
	let x = Millimeters(125) + Meters(3048);
	println!("{:?}", x);


	let y = Meters(12);
	println!("{:?}", AmbiguousTrait::coolify(&y));
	println!("{:?}", UnpredictableTrait::coolify(&y));

}