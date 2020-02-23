use crate::points::{Point, Vector};

use std::vec::Vec;


#[derive(Debug, PartialEq)]
pub struct PolygonVertex<'a>{
	vertices: &'a Vec<Point>,
	index: usize,
}

#[derive(Debug, PartialEq)]
pub struct PolygonEdge(Point, Point);

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


impl PolygonEdge {
	fn direction(&self) -> Vector {
		self.1 - self.0
	}

	fn region(&self, point: Point) -> EdgeRegion {
		use std::cmp::Ordering::*;
		use EdgeRegion::*;

		match self.direction()
			.normal()
			.dot(point-self.0)
			.partial_cmp(&0.).unwrap()
		{
			Less => Exterior,
			Equal => Boundary,
			Greater => Interior,
		}
	}
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

	pub fn fwd_edge(&self) -> PolygonEdge {
		PolygonEdge(
			self.vertices[self.index],
			self.vertices[(self.index+1) % self.vertices.len()]
		)
	}

	pub fn rev_edge(&self) -> PolygonEdge {
		PolygonEdge(
			self.vertices[(self.index+self.vertices.len()-1) % self.vertices.len()],
			self.vertices[self.index]
		)
	}

	pub fn to_id(self) -> usize {
		self.index
	}
}


impl ConvexPolygon {
	pub fn new() -> Self {
		Self{vertices: Vec::new()}
	}

	pub fn degree(&self) -> usize {
		self.vertices.len()
	}

	fn vertex(&self, index: usize) -> PolygonVertex {
		PolygonVertex {
			vertices: &self.vertices,
			index,
		}
	}

	pub fn some_vertex(&self) -> PolygonVertex {
		self.vertex(0_usize)
	}

	fn exterior_witness(&self, point: Point) -> Option<PolygonVertex> {
		(0..self.vertices.len())
			.map(|i| self.vertex(i))
			.filter(|v| v.fwd_edge().region(point) == EdgeRegion::Exterior)
			.next()
	}

	pub fn covers(&self, point: Point) -> bool {
		self.exterior_witness(point).is_none()
	}

	pub fn find(&self, point: Point) -> Option<PolygonVertex> {
		Some(self.vertex(
			(0..self.vertices.len())
			.filter(|i| self.vertices[*i] == point)
			.next()?
		))
	}

	pub fn find_best(&self, objective: Vector) -> PolygonVertex {
		(0..self.vertices.len())
			.map(|i| self.vertex(i))
			.filter(|v|
				objective.dot(v.fwd_edge().direction()) <= 0.
				&& objective.dot(v.rev_edge().direction()) >= 0.
			)
			.next().unwrap()
	}

	pub fn insert(&mut self, new_point: Point) -> Vec<Point> {
		if let Some(vertex) = self.exterior_witness(new_point) {
			let n = self.vertices.len();

			let v0_idx = (0..n)
				.map(|j| (n+vertex.index-j)%n)
				.filter(|j| {
					self.vertex(*j).rev_edge().region(new_point)
					== EdgeRegion::Interior
				})
				.next().unwrap();
			let v1_idx = (0..n)
				.map(|j| (vertex.index+1+j)%n)
				.filter(|j| {
					self.vertex(*j).fwd_edge().region(new_point)
					== EdgeRegion::Interior
				})
				.next().unwrap();

			assert!(v0_idx != v1_idx);

			let mut removed_vertices = Vec::<Point>::new();
			if v0_idx < v1_idx {
				removed_vertices.extend(self.vertices.drain(v0_idx+1..v1_idx));
				self.vertices.insert(v0_idx+1, new_point);
			} else {
				removed_vertices.extend(self.vertices.drain(v0_idx+1..));
				removed_vertices.extend(self.vertices.drain(..v1_idx));
				self.vertices.push(new_point);
			}

			removed_vertices
		} else if self.degree() <= 1 && self.find(new_point).is_none() {
			self.vertices.push(new_point);
			vec!()
		} else {
			vec!(new_point)
		}
	}

	pub fn remove(&mut self, index: usize) -> Point {
		self.vertices.remove(index)
	}
}


///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::{ConvexPolygon, PolygonVertex, PolygonEdge, Point, Vector};

	fn convex_polygon() -> ConvexPolygon {
		let mut cp = ConvexPolygon::new();
		cp.insert(Point{x: 0., y: 1.});
		cp.insert(Point{x: 2., y: 0.});
		cp.insert(Point{x: -1., y: -1.});
		cp.insert(Point{x: 1., y: -1.});
		assert_eq!(cp.degree(), 4);

		cp
	}

	fn start_vertex<'a>(cp: &'a ConvexPolygon) -> PolygonVertex<'a> {
		cp.find(Point{x: 0., y: 1.}).unwrap()
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
		let _cp = ConvexPolygon::new();
	}

	#[test]
	fn length() {
		let cp = convex_polygon();
		assert_eq!(cp.degree(), 4_usize);  // fill w/ actual length
	}

	#[test]
	fn vertex() {
		let cp = convex_polygon();
		let mut vertex = start_vertex(&cp);
		let ordered_vertices = ordered_vertices();

		for i in 0..4 {
			assert_eq!(vertex.position(), ordered_vertices[i]);
			vertex = vertex.fwd_vertex();
		}
	}

	#[test]
	fn fwd_edge() {
		let cp = convex_polygon();
		let mut vertex = start_vertex(&cp);
		let ordered_vertices = ordered_vertices();

		for i in 0..4 {
			assert_eq!(
				vertex.fwd_edge(),
				PolygonEdge(ordered_vertices[i], ordered_vertices[(i+1) % 4])
			);
			vertex = vertex.fwd_vertex();
		}
	}

	#[test]
	fn rev_edge() {
		let cp = convex_polygon();
		let mut vertex = start_vertex(&cp);
		let ordered_vertices = ordered_vertices();

		for i in 0_usize..4_usize {
			vertex = vertex.fwd_vertex();
			assert_eq!(
				vertex.rev_edge(),
				PolygonEdge(ordered_vertices[i], ordered_vertices[(i+1) % 4])
			);
		}
	}

	#[test]
	fn covers() {
		let cp = convex_polygon();

		assert!(cp.covers(Point{x: 1., y: 0.}));
		assert!(cp.covers(Point{x: 1., y: 0.5}));
		assert!(!cp.covers(Point{x: 1., y: 1.}));
	}

	#[test]
	fn find() {
		let cp = convex_polygon();
		let mut vertex = start_vertex(&cp);
		let ordered_vertices = ordered_vertices();

		for i in 0_usize..4_usize {
			let next_vertex = vertex.fwd_vertex();
			assert_eq!(
				cp.find(ordered_vertices[i]),
				Some(vertex),
			);
			vertex = next_vertex;
		}
		assert_eq!(cp.find(Point{x: 0., y: 0.}), None);
	}

	#[test]
	fn find_best() {
		let cp = convex_polygon();
		assert_eq!(
			cp.find_best(Vector{x: 2., y: 0.}).position(),
			Point{x: 2., y: 0.},
		);
		assert_eq!(
			cp.find_best(Vector{x: 1., y: 0.}).position(),
			Point{x: 2., y: 0.},
		);
		assert_eq!(
			cp.find_best(Vector{x: 3., y: 1.}).position(),
			Point{x: 2., y: 0.},
		);
	}

	#[test]
	fn insert() {
		let mut cp = convex_polygon();
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
		let mut cp = convex_polygon();
		let vertex = start_vertex(&cp);
		let ordered_vertices = ordered_vertices();

		let vertex_id = vertex.to_id();
		assert_eq!(cp.remove(vertex_id), ordered_vertices[0]);
	}
}
