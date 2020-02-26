use crate::points::{Point, Vector};
use crate::polygon::ConvexPolygon;
use crate::circle::ClosedCircle;

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

	pub fn len(&self) -> usize {
		assert_eq!(self.point_log.len(), self.cover.degree() + self.interior.len());
		self.point_log.len()
	}

	pub fn extend<I: Iterator<Item=Point>>(&mut self, iter: I) {
		for p in iter {
			self.push(p);
		}
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

	pub fn cover_circle(&self) -> ClosedCircle {
		match self.cover.degree() {
			0 => return ClosedCircle {
				center: Point{x: std::f64::NAN, y: std::f64::NAN},
				sq_radius: 0.
			},
			1 => return ClosedCircle {
				center: self.cover.some_vertex().position(),
				sq_radius: 0.
			},
			_ => {}
		};

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

		let find_farthest_pos = |ignores: HashSet<(u64, u64)>| {
			let mut vertex = self.cover.some_vertex();
			let mut best_pos: Option<Point> = None;
			
			for _i in 0..self.cover.degree() {
				let pos = vertex.position();
				vertex = vertex.fwd_vertex();

				if ignores.contains(&pos.to_bits()) {
					continue;
				}
				best_pos = match best_pos {
					None => Some(pos),
					Some(b_pos) => Some({
						if (pos-center).sq_mag() > (b_pos-center).sq_mag() {
							pos
						} else {
							b_pos
						}
					}),
				};
			}

			best_pos
		};

		let circ_pos1 = find_farthest_pos(HashSet::<(u64, u64)>::new()).unwrap();
		let circ_pos2 = find_farthest_pos(
			vec![circ_pos1]
			.into_iter().map(|p| p.to_bits()).collect()
		).unwrap();
		let circ_2pt = ClosedCircle::from_two_points(circ_pos1, circ_pos2);

		let circ_pos3 = find_farthest_pos(
			vec![circ_pos1, circ_pos2]
			.into_iter().map(|p| p.to_bits()).collect()
		);

		match circ_pos3 {
			Some(p3) => {
				if !circ_2pt.covers(p3) {
					ClosedCircle::from_three_points(circ_pos1, circ_pos2, p3)
				} else {
					circ_2pt
				}
			},
			None => circ_2pt,
		}
	}
}
