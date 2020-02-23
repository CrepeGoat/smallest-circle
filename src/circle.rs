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
		// see http://ambrsoft.com/TrigoCalc/Circle3D.htm
		let origin = Point::default();
		let denominator = 2. * (
			p1.x * (p2.y-p3.y)
			- p1.y * (p2.x-p3.x)
			+ p2.x*p3.y
			- p2.y*p3.x
		);
		let center = Point{
			x: (
				(p1-origin).sq_mag() * (p2.y-p3.y)
				+ (p2-origin).sq_mag() * (p3.y-p1.y)
				+ (p3-origin).sq_mag() * (p1.y-p2.y)
			) / denominator,
			y: (
				(p1-origin).sq_mag() * (p3.x-p2.x)
				+ (p2-origin).sq_mag() * (p1.x-p3.x)
				+ (p3-origin).sq_mag() * (p2.x-p1.x)
			) / denominator,
		};
		
		Self {center, sq_radius: (p1-center).sq_mag()}

	}

	pub fn covers(&self, point: Point) -> bool {
		(point - self.center).sq_mag() <= self.sq_radius
	}

	pub fn radius(&self) -> f64 {
		f64::sqrt(self.sq_radius)
	}
}


///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::{ClosedCircle, Point};

	#[test]
	fn from_two_points() {
		let p1 = Point{x: 4., y: -1.};
		let p2 = Point{x: 0., y: -1.};

		assert_eq!(
			ClosedCircle::from_two_points(p1, p2),
			ClosedCircle{
				center: Point{x: 2., y: -1.},
				sq_radius: 4.
			}
		);

		// Permutation of points shouldn't change result
		assert_eq!(
			ClosedCircle::from_two_points(p1, p2),
			ClosedCircle::from_two_points(p2, p1),
		);
	}

	#[test]
	fn from_three_points() {
		let p1 = Point{x: 4., y: -1.};
		let p2 = Point{x: 0., y: -1.};
		let p3 = Point{x: 2., y: 1.};

		assert_eq!(
			ClosedCircle::from_three_points(p1, p2, p3),
			ClosedCircle{
				center: Point{x: 2., y: -1.},
				sq_radius: 4.
			}
		);

		// Permutation of points shouldn't change result
		assert_eq!(
			ClosedCircle::from_three_points(p1, p2, p3),
			ClosedCircle::from_three_points(p3, p2, p1),
		);
	}

	#[test]
	fn covers() {
		let circle = ClosedCircle{
			center: Point{x: 2., y: -1.},
			sq_radius: 4.,
		};

		assert!(circle.covers(Point{x: 2., y: -1.}));
		assert!(circle.covers(Point{x: 1., y: 0.}));
		assert!(circle.covers(Point{x: 0., y: -1.}));
		assert!(!circle.covers(Point{x: 0., y: 0.}));
	}

	#[test]
	fn radius() {
		let circle = ClosedCircle{
			center: Point{x: 2., y: -1.},
			sq_radius: 4.,
		};

		assert_eq!(circle.radius(), 2.);
	}
}
