use std::fmt;
use std::f64;


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point{pub x: f64, pub y: f64}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
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


///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::Point;

    #[test]
    fn make_point() {
		let p1 = Point{x: 1.0, y: 2.0};
    }

    #[test]
    fn convert_through_bits() {
		let p1 = Point{x: 1.0, y: -2.0};
		assert_eq!(p1, Point::from_bits(p1.to_bits()));
    }

}
