use crate::points::Point;

use std::vec::Vec;


#[derive(Debug, PartialEq)]
pub struct PolygonVertex<'a>{
	vertices: &'a Vec<Point>,
	index: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConvexPolygon{
	vertices: Vec<Point>,
}

#[derive(Debug, PartialEq)]
enum EdgeRegion {
	Interior,
	Boundary,
	Exterior,
}


impl PolygonVertex<'_> {
	pub fn position(&self) -> Point {
		self.vertices[self.index]
	}

	pub fn fwd_vertex(&self) -> Self {
		Self {
			index: (self.index+1) % self.vertices.len(),
			..*self
		}
	}
	
	pub fn rev_vertex(&self) -> Self {
		Self {
			index: (self.index+self.vertices.len()-1) % self.vertices.len(),
			..*self
		}
	}
}


impl ConvexPolygon {
	pub fn new() -> Self {
		Self{vertices: Vec::new()}
	}

	pub fn degree(&self) -> usize {
		self.vertices.len()
	}

	pub fn vertex(&self, index: usize) -> Point {
		self.vertices[index]
	}

	pub fn fwd_edge(&self, index: usize) -> (Point, Point) {
		(
			self.vertices[index],
			self.vertices[(index+1) % self.vertices.len()]
		)
	}

	pub fn rev_edge(&self, index: usize) -> (Point, Point) {
		(
			self.vertices[(index+self.vertices.len()-1) % self.vertices.len()],
			self.vertices[index]
		)
	}

	fn region(point: Point, edge: (Point, Point)) -> EdgeRegion {
		use std::cmp::Ordering::*;
		use EdgeRegion::*;

		match (edge.1-edge.0)
			.normal()
			.dot(point-edge.0)
			.partial_cmp(&0.).unwrap()
		{
			Less => Exterior,
			Equal => Boundary,
			Greater => Interior,
		}
	}

	fn exterior_witness(&self, point: Point) -> Option<usize> {
		(0..self.vertices.len())
			.filter(|i| {
				Self::region(point, self.fwd_edge(*i)) == EdgeRegion::Exterior
			})
			.next()
	}

	pub fn covers(&self, point: Point) -> bool {
		self.exterior_witness(point).is_none()
	}

	pub fn find(&self, vertex: Point) -> Option<usize> {
		(0..self.vertices.len())
			.filter(|i| self.vertex(*i) == vertex)
			.next()
	}

	pub fn insert(&mut self, new_vertex: Point) -> Vec<Point> {
		if let Some(i) = self.exterior_witness(new_vertex) {
			let n = self.vertices.len();

			let v0_idx = (0..n)
				.map(|j| (n+i-j)%n)
				.filter(|j| {
					Self::region(new_vertex, self.rev_edge(*j))
					== EdgeRegion::Interior
				})
				.next().unwrap();
			let v1_idx = (0..n)
				.map(|j| (i+1+j)%n)
				.filter(|j| {
					Self::region(new_vertex, self.fwd_edge(*j))
					== EdgeRegion::Interior
				})
				.next().unwrap();

			assert!(v0_idx != v1_idx);

			let mut removed_vertices = Vec::<Point>::new();
			if v0_idx < v1_idx {
				removed_vertices.extend(self.vertices.drain(v0_idx+1..v1_idx));
				self.vertices.insert(v0_idx+1, new_vertex);
			} else {
				removed_vertices.extend(self.vertices.drain(v0_idx+1..));
				removed_vertices.extend(self.vertices.drain(..v1_idx));
				self.vertices.push(new_vertex);
			}

			removed_vertices
		} else if self.degree() <= 1 && self.find(new_vertex).is_none() {
			self.vertices.push(new_vertex);
			vec!()
		} else {
			vec!(new_vertex)
		}
	}

	pub fn remove(&mut self, index: usize) -> Point {
		self.vertices.remove(index)
	}
}


///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::{ConvexPolygon, Point};

	fn convex_polygon() -> (ConvexPolygon, usize) {
		let mut cp = ConvexPolygon::new();
		cp.insert(Point{x: 0., y: 1.});
		cp.insert(Point{x: 2., y: 0.});
		cp.insert(Point{x: -1., y: -1.});
		cp.insert(Point{x: 1., y: -1.});
		assert_eq!(cp.degree(), 4);

		let start_index = cp.find(Point{x: 0., y: 1.}).unwrap();

		(cp, start_index)
	}

	fn ordered_vertices() -> Vec<Point> {
		vec!(
			Point{x: 0., y: 1.},
			Point{x: -1., y: -1.},
			Point{x: 1., y: -1.},
			Point{x: 2., y: 0.},
		)
	}

	#[test]
	fn new() {
		let cp = ConvexPolygon::new();
	}

	#[test]
	fn length() {
		let cp = convex_polygon().0;
		assert_eq!(cp.degree(), 4_usize);  // fill w/ actual length
	}

	#[test]
	fn vertex() {
		let (cp, start_index) = convex_polygon();
		let ordered_vertices = ordered_vertices();

		for i in 0_usize..4_usize {
			assert_eq!(cp.vertex((start_index+i) % 4), ordered_vertices[i]);
		}
	}

	#[test]
	fn fwd_edge() {
		let (cp, start_index) = convex_polygon();
		let ordered_vertices = ordered_vertices();

		for i in 0_usize..4_usize {
			assert_eq!(
				cp.fwd_edge((start_index+i) % 4),
				(ordered_vertices[i], ordered_vertices[(i+1) % 4])
			);
		}
	}

	#[test]
	fn rev_edge() {
		let (cp, start_index) = convex_polygon();
		let ordered_vertices = ordered_vertices();

		for i in 0_usize..4_usize {
			assert_eq!(
				cp.rev_edge((start_index+i+1) % 4),
				(ordered_vertices[i], ordered_vertices[(i+1) % 4])
			);
		}
	}

	#[test]
	fn covers() {
		let cp = convex_polygon().0;

		assert!(cp.covers(Point{x: 1., y: 0.}));
		assert!(cp.covers(Point{x: 1., y: 0.5}));
		assert!(!cp.covers(Point{x: 1., y: 1.}));
	}

	#[test]
	fn find() {
		let (cp, start_index) = convex_polygon();
		let ordered_vertices = ordered_vertices();

		for i in 0_usize..4_usize {
			assert_eq!(cp.find(ordered_vertices[i]), Some((start_index+i) % 4));
		}
		assert_eq!(cp.find(Point{x: 0., y: 0.}), None);
	}

	#[test]
	fn insert() {
		let mut cp = convex_polygon().0;
		// Add interior point
		{
			let point = Point{x: 0., y: 0.};
			assert!(cp.covers(point));

			let total_count = cp.degree() + 1;
			let removed_vertices = cp.insert(point);
			assert_eq!(removed_vertices, vec!(point));
			assert_eq!(cp.degree() + removed_vertices.len(), total_count);
		}
		// Add exterior point that obviates an existing vertex
		{
			let point = Point{x: 0., y: 2.};
			assert!(!cp.covers(point));
			let total_count = cp.degree() + 1;

			let removed_vertices = cp.insert(point);
			assert_eq!(cp.degree() + removed_vertices.len(), total_count);
			assert_eq!(removed_vertices, vec!(Point{x: 0., y: 1.}));
			assert!(cp.find(point).is_some());
			assert!(cp.find(removed_vertices[0]).is_none());
		}
		// Add exterior point w/o obviating existing vertices
		{
			let point = Point{x: 2., y: 2.};
			assert!(!cp.covers(point));
			let total_count = cp.degree() + 1;

			let removed_vertices = cp.insert(point);
			assert_eq!(cp.degree() + removed_vertices.len(), total_count);
			assert_eq!(removed_vertices, vec!());
			assert!(cp.find(point).is_some());
		}
	}

	#[test]
	fn remove() {
		let (mut cp, start_index) = convex_polygon();
		let ordered_vertices = ordered_vertices();
		assert_eq!(cp.remove(start_index), ordered_vertices[0]);
	}
}
