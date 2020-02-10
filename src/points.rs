use std::fmt;
use std::ops::*;


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point{pub x: f64, pub y: f64}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector{pub x: f64, pub y: f64}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<{}, {}>", self.x, self.y)
    }
}

impl Point {
	pub fn to_bits(self) -> (u64, u64) {
		(self.x.to_bits(), self.y.to_bits())
	}

	pub fn from_bits(bits: (u64, u64)) -> Self {
		Self{
			x: f64::from_bits(bits.0),
			y: f64::from_bits(bits.1),
		}
	}
}



//-----------------------------------------------------------------------------
// Additions
//-----------------------------------------------------------------------------

// Point + Point
// cannot add two points

// Point + Vector
impl Add<Vector> for Point {
	type Output = Self;
	
	fn add(self, other: Vector) -> Self::Output {
		Self{x: self.x + other.x, y: self.y + other.y}
	}
}

// Vector + Point
// cannot add a point to a vector (only vice-versa)

// Vector + Vector
impl Add<Vector> for Vector {
	type Output = Self;
	
	fn add(self, other: Self) -> Self::Output {
		Self{x: self.x + other.x, y: self.y + other.y}
	}
}


//-----------------------------------------------------------------------------
// Subtractions
//-----------------------------------------------------------------------------

// Point - Point
impl Sub<Point> for Point {
	type Output = Vector;
	
	fn sub(self, other: Self) -> Self::Output {
		Vector{x: self.x - other.x, y: self.y - other.y}
	}
}

// Point - Vector
impl Sub<Vector> for Point {
	type Output = Point;
	
	fn sub(self, other: Vector) -> Self::Output {
		Point{x: self.x - other.x, y: self.y - other.y}
	}
}

// Vector - Point
// cannot subtract a point from a vector

// Vector - Vector
impl Sub<Vector> for Vector {
	type Output = Self;
	
	fn sub(self, other: Self) -> Self::Output {
		Self{x: self.x - other.x, y: self.y - other.y}
	}
}


//-----------------------------------------------------------------------------
// Multiplications
//-----------------------------------------------------------------------------

impl Mul<f64> for Vector {
	type Output = Self;
	
	fn mul(self, other: f64) -> Self::Output {
		Self{x: self.x * other, y: self.y * other}
	}
}

//-----------------------------------------------------------------------------
// Vector Operations
//-----------------------------------------------------------------------------

impl Vector {
	pub fn dot(self, other: Self) -> f64 {
		self.x*other.x + self.y*other.y
	}

	pub fn cross(self, other: Self) -> f64 {
		self.x*other.y - self.y*other.x	
	}

	pub fn rotated(self, angle: f64) -> Vector {
		Vector{
			x: self.x*angle.cos() + -self.y*angle.sin(),
			y: self.x*angle.sin() + self.y*angle.cos(),
		}
	}

	pub fn rotated_quantage(self, quantage: f64) -> Vector {
		self.rotated(quantage * 2.*std::f64::consts::PI)
	}
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::{Point, Vector};

    #[test]
    fn make_point() {
		let p1 = Point{x: 1.0, y: 2.0};
    }

    #[test]
    fn make_vector() {
		let p1 = Vector{x: 1.0, y: 2.0};
    }

    #[test]
    fn convert_through_bits() {
		let p1 = Point{x: 1.0, y: -2.0};
		assert_eq!(p1, Point::from_bits(p1.to_bits()));
    }

    //-------------------------------------------------------------------------

    #[test]
    fn p_add_v() {
		let p = Point{x: 1., y: 2.};
		let v = Vector{x: 3., y: 5.};
		assert_eq!(p+v, Point{x: 4., y: 7.});
    }

    #[test]
    fn v_add_v() {
		let v1 = Vector{x: 1., y: 2.};
		let v2 = Vector{x: 3., y: 5.};
		assert_eq!(v1+v2, Vector{x: 4., y: 7.});
    }

    #[test]
    fn p_sub_p() {
		let p1 = Point{x: 1., y: 2.};
		let p2 = Point{x: 3., y: 5.};
		assert_eq!(p2-p1, Vector{x: 2., y: 3.});
    }

    #[test]
    fn p_sub_v() {
		let v = Vector{x: 1., y: 2.};
		let p = Point{x: 3., y: 5.};
		assert_eq!(p-v, Point{x: 2., y: 3.});
    }

    #[test]
    fn v_sub_v() {
		let v1 = Vector{x: 1., y: 2.};
		let v2 = Vector{x: 3., y: 5.};
		assert_eq!(v2-v1, Vector{x: 2., y: 3.});
    }

    #[test]
    fn v_mul_s() {
		let v = Vector{x: 1., y: 2.};
		assert_eq!(v*2., Vector{x: 2., y: 4.});
    }

	#[test]
	fn v_dot_v() {
		let v1 = Vector{x: 1., y: 2.};
		let v2 = Vector{x: -3., y: 5.};
		assert_eq!(v1.dot(v2), 7.);
	}

	#[test]
	fn v_cross_v() {
		let v1 = Vector{x: 1., y: 2.};
		let v2 = Vector{x: -3., y: 5.};
		assert_eq!(v1.cross(v2), 11.);
	}

	#[test]
	fn v_rotated() {
		use std::f64::consts::PI;
		let v = Vector{x: 1., y: 2.};
		assert_eq!(v.rotated(0.), Vector{x: 1., y: 2.});
		assert_eq!(v.rotated(0.5*PI), Vector{x: -2., y: 1.0000000000000002});
		assert_eq!(v.rotated(PI), Vector{x: -1.0000000000000002, y: -1.9999999999999998});
		assert_eq!(v.rotated(1.5*PI), Vector{x: 1.9999999999999998, y: -1.0000000000000004});
		assert_eq!(v.rotated(2.*PI), Vector{x: 1.0000000000000004, y: 1.9999999999999998});
	}

	#[test]
	fn v_rotated_quantage() {
		let v = Vector{x: 1., y: 2.};
		assert_eq!(v.rotated_quantage(0.), Vector{x: 1., y: 2.});
		assert_eq!(v.rotated_quantage(0.25), Vector{x: -2., y: 1.0000000000000002});
		assert_eq!(v.rotated_quantage(0.5), Vector{x: -1.0000000000000002, y: -1.9999999999999998});
		assert_eq!(v.rotated_quantage(0.75), Vector{x: 1.9999999999999998, y: -1.0000000000000004});
		assert_eq!(v.rotated_quantage(1.), Vector{x: 1.0000000000000004, y: 1.9999999999999998});
	}
}
