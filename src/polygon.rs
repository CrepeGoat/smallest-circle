use crate::points::Point;

use std::vec::Vec;


#[derive(Debug, PartialEq)]
pub struct ConvexPolygon{
	vertices: Vec<Point>,
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

	pub fn covers(&self, point: Point) -> bool {
		unimplemented!();
	}

	pub fn find(&self, vertex: Point) -> Option<usize> {
		unimplemented!();
	}

	pub fn insert(&mut self, vertex: Point) -> Vec<Point> {
		unimplemented!();
	}

	pub fn remove(&mut self, index: usize) -> Point {
		unimplemented!();
	}
}


///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::{ConvexPolygon, Point};

	fn convex_polygon() -> ConvexPolygon {
		unimplemented!();
	}

	#[test]
	fn new() {
		let cp = ConvexPolygon::new();
	}

	#[test]
	fn length() {
		let cp = convex_polygon();
		assert_eq!(cp.degree(), 0_usize);  // fill w/ actual length
	}

	#[test]
	fn vertex() {
		let cp = convex_polygon();
		assert_eq!(cp.vertex(0_usize), Point::default());  // fill w/ actual points
	}

	#[test]
	fn fwd_edge() {
		let cp = convex_polygon();
		assert_eq!(
			cp.fwd_edge(0_usize),
			(Point::default(), Point::default())  // fill w/ actual points
		);
	}

	#[test]
	fn rev_edge() {
		let cp = convex_polygon();
		assert_eq!(
			cp.rev_edge(0_usize),
			(Point::default(), Point::default())  // fill w/ actual points
		);
	}

	#[test]
	fn covers() {
		let cp = convex_polygon();
		assert!(cp.covers(Point::default()));  // fill w/ actual points
		assert!(!cp.covers(Point::default()));  // fill w/ actual points
	}

	#[test]
	fn find() {
		let cp = convex_polygon();
		assert_eq!(cp.find(Point::default()), Some(0_usize));  // fill w/ actual points
		assert_eq!(cp.find(Point::default()), None);  // fill w/ actual points
	}

	#[test]
	fn insert() {
		let mut cp = convex_polygon();
		assert_eq!(cp.insert(Point::default()), vec!(Point::default()));  // fill w/ actual points
	}

	#[test]
	fn remove() {
		let mut cp = convex_polygon();
		assert_eq!(cp.remove(0_usize), Point::default());  // fill w/ actual points
	}
}
