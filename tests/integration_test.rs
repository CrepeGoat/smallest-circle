use smallest_circle::smallest_cover;
use smallest_circle::points::Point;


#[test]
fn point_cloud_use() {
	let mut point_cloud = smallest_cover::PointCloud::new();

	// Populate initial points
	{
		let dth = 120_f64.to_radians();
		let theta0 = 1_f64;
		let r = 1.;
		
		point_cloud.extend(vec!(
			Point{x: r*theta0.cos(), y: r*theta0.sin()},
			Point{x: r*(theta0+dth).cos(), y: r*(theta0+dth).sin()},
			Point{x: r*(theta0+2.*dth).cos(), y: r*(theta0+2.*dth).sin()},
		).into_iter());
	};
	assert!(point_cloud.cover_radius() == 1.);

	// Add interior points
	{
		let dth = 90_f64.to_radians();
		let theta0 = 0_f64;
		let r = 0.5;

		point_cloud.insert(Point{x: r*theta0.cos(), y: r*theta0.sin()});
		point_cloud.insert(Point{x: r*(theta0+dth).cos(), y: r*(theta0+dth).sin()});
		point_cloud.insert(Point{x: r*(theta0+2.*dth).cos(), y: r*(theta0+2.*dth).sin()});
		point_cloud.insert(Point{x: r*(theta0+3.*dth).cos(), y: r*(theta0+3.*dth).sin()});
	};
	assert!(point_cloud.cover_radius() == 1.);

	// Remove original polygon vertices
	{
		let dth = 120_f64.to_radians();
		let theta0 = 1_f64;
		let r = 1.;
		
		point_cloud.remove(Point{x: r*theta0.cos(), y: r*theta0.sin()});
		point_cloud.remove(Point{x: r*(theta0+dth).cos(), y: r*(theta0+dth).sin()});
		point_cloud.remove(Point{x: r*(theta0+2.*dth).cos(), y: r*(theta0+2.*dth).sin()});
	};
	assert!(point_cloud.cover_radius() == 0.5);


	// Add new polygon vertices, w/o changing radius
	point_cloud.insert(Point{x: 0.3, y: 0.3});
	assert!(point_cloud.cover_radius() == 0.5);

	// Remove all but one point
	{
		let dth = 90_f64.to_radians();
		let theta0 = 0_f64;
		let r = 0.5;

		point_cloud.remove(Point{x: r*theta0.cos(), y: r*theta0.sin()});
		point_cloud.remove(Point{x: r*(theta0+dth).cos(), y: r*(theta0+dth).sin()});
		point_cloud.remove(Point{x: r*(theta0+2.*dth).cos(), y: r*(theta0+2.*dth).sin()});
		point_cloud.remove(Point{x: r*(theta0+3.*dth).cos(), y: r*(theta0+3.*dth).sin()});
	};
	assert!(point_cloud.cover_radius() == 0.);
}
