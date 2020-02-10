use std::fmt;


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point{pub x: f64, pub y: f64}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}


///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn make_point() {
		let _p = super::Point{x: 1.0, y: 2.0};
    }
}
