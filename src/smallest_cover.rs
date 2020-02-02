use std::marker::PhantomData;

use std::iter::Iterator;

pub struct PointCloud<POINT>{
	dummy: PhantomData<POINT>
}

impl<POINT> PointCloud<POINT> {
	pub fn new() -> Self {
		Self{dummy: PhantomData}
	}

	pub fn extend<I: Iterator<Item=POINT>>(&mut self, iter: I) {
		unimplemented!();
	}

	pub fn insert(&mut self, p: POINT) {
		unimplemented!();
	}

	pub fn remove(&mut self, p: POINT) {
		unimplemented!();
	}

	pub fn cover_radius(&self) -> f64 {
		unimplemented!();
	}
}
