use crate::points::Point;


#[derive(Debug, PartialEq)]
pub struct ClosedCircle {
	pub center: Point,
	pub sq_radius: f64,
}


impl ClosedCircle {
	pub fn from_two_points(p1: Point, p2: Point) -> Self {
		Self {
			center: p1 + (p2-p1)*0.5,
			sq_radius: 0.25*(p2-p1).sq_mag()
		}
	}

	pub fn from_three_points(p1: Point, p2: Point, p3: Point) -> Self {
		unimplemented!()
	}

	pub fn covers(&self, point: Point) -> bool {
		(point - self.center).sq_mag() <= self.sq_radius
	}

	pub fn radius(&self) -> f64 {
		f64::sqrt(self.sq_radius)
	}
}