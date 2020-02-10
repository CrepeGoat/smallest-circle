use crate::points::Point;

use std::iter::Iterator;

pub struct PointCloud{}

impl PointCloud {
	pub fn new() -> Self {
		Self{}
	}

	pub fn extend<I: Iterator<Item=Point>>(&mut self, iter: I) {
		unimplemented!();
	}

	pub fn insert(&mut self, p: Point) {
		unimplemented!();
	}

	pub fn remove(&mut self, p: Point) {
		unimplemented!();
	}

	pub fn cover_radius(&self) -> f64 {
		unimplemented!();
	}
}
