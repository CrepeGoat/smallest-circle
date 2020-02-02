use crate::points::Point;
use crate::polygon::ConvexPolygon;

use std::iter::Iterator;
use std::collections::HashSet;
use std::collections::VecDeque;


pub struct MovingPointCloud{
	cover: ConvexPolygon,
	interior: HashSet<(u64, u64)>,
	point_log: VecDeque<Point>,
}

impl MovingPointCloud {
	pub fn new() -> Self {
		Self{
			cover: ConvexPolygon::new(),
			interior: HashSet::new(),
			point_log: VecDeque::new(),
		}
	}

	pub fn extend<I: Iterator<Item=Point>>(&mut self, iter: I) {
		unimplemented!();
	}

	pub fn push(&mut self, p: Point) {
		self.point_log.push_back(p);
		for p in self.cover.insert(p).into_iter() {
			self.interior.insert(p.to_bits());
		}
	}

	pub fn pop(&mut self) {
		if let Some(p) = self.point_log.pop_front() {
			if self.interior.remove::<(u64, u64)>(&p.to_bits()) {
				return;
			}

			if let Some(i) = self.cover.find(p) {
				let mut lost_area = ConvexPolygon::new();
				lost_area.insert(self.cover.rev_edge(i).0);
				lost_area.insert(self.cover.fwd_edge(i).1);
				lost_area.insert(self.cover.remove(i));

				for p in self.interior.iter()
					.map(|b| Point::from_bits(*b))
					.filter(|p| lost_area.covers(*p))
				{
					self.cover.insert(p);
				}
			}
		}
	}

	pub fn cover_radius(&self) -> f64 {
		unimplemented!();
	}
}
