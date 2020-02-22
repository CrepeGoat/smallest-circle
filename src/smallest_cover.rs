use crate::points::{Point, Vector};
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
			if self.interior.remove::<(u64, u64)>(&p.to_bits()) {}

			else if let Some(vertex) = self.cover.find(p) {
				let mut lost_area = ConvexPolygon::new();
				lost_area.insert(vertex.rev_vertex().position());
				lost_area.insert(vertex.fwd_vertex().position());
				let vertex_id = vertex.to_id();
				lost_area.insert(self.cover.remove(vertex_id));

				for p in self.interior.iter()
					.map(|b| Point::from_bits(*b))
					.filter(|p| lost_area.covers(*p))
				{
					self.cover.insert(p);
				}
			}

			else {
				unreachable!();
			}
		}
	}

	pub fn cover_radius(&self) -> f64 {
		if self.cover.degree() <= 1 {
			return 0.
		}

		// Calculate square center
		let center = {
			let x_bounds = (
				self.cover.find_best(Vector{x: -1., y: 0.}).position().x,
				self.cover.find_best(Vector{x: 1., y: 0.}).position().x,
			);
			let y_bounds = (
				self.cover.find_best(Vector{x: 0., y: -1.}).position().y,
				self.cover.find_best(Vector{x: 0., y: 1.}).position().y,
			);

			Point{
				x: (x_bounds.0+x_bounds.1) / 2.,
				y: (y_bounds.0+y_bounds.1) / 2.,
			}
		};

		let circumfrence_point1 = self.cover.iter()
			.max_by_key(|v| (v.position() - center).sq_mag());
		let circumfrence_point2 = self.cover.iter()
			.filter(|v| v != circumfrence_point1)
			.max_by_key(|v| (v.position() - center).sq_mag());

		unimplemented!();
	}
}
