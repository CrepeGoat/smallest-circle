use crate::points::Point;
use crate::polygon::ConvexPolygon;

use std::iter::Iterator;
use std::collections::HashSet;


pub struct PointCloud{
	cover: ConvexPolygon,
	interior: HashSet<Point>,
}

impl PointCloud {
	pub fn new() -> Self {
		Self{
			cover: ConvexPolygon::new(),
			interior: HashSet::new(),
		}
	}

	pub fn extend<I: Iterator<Item=Point>>(&mut self, iter: I) {
		unimplemented!();
	}

	pub fn insert(&mut self, p: Point) {
		for p in self.cover.insert(p).into_iter() {
			self.interior.insert(p);
		}
	}

	pub fn remove(&mut self, p: Point) {
		if self.interior.remove(&p) {
			return;
		}

		if let Some(i) = self.cover.find(p) {
			let lost_area = ConvexPolygon::new();
			lost_area.insert(self.cover.rev_edge(i).0);
			lost_area.insert(self.cover.fwd_edge(i).1);
			lost_area.insert(self.cover.remove(i));

			for p in self.interior.iter().filter(|p| lost_area.covers(p)) {
				self.cover.insert(*p);
			}
		}
	}

	pub fn cover_radius(&self) -> f64 {
		unimplemented!();
	}
}
