use smallest_circle::smallest_cover;
use smallest_circle::points::Point;


#[test]
fn point_cloud_use() {
	let mut point_cloud = smallest_cover::MovingPointCloud::new();

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
	assert_eq!(point_cloud.cover_circle().radius(), 1.);
	assert_eq!(point_cloud.len(), 3);

	// Add interior points
	{
		let dth = 90_f64.to_radians();
		let theta0 = 0_f64;
		let r = 0.5;

		point_cloud.push(Point{x: r*theta0.cos(), y: r*theta0.sin()});
		point_cloud.push(Point{x: r*(theta0+dth).cos(), y: r*(theta0+dth).sin()});
		point_cloud.push(Point{x: r*(theta0+2.*dth).cos(), y: r*(theta0+2.*dth).sin()});
		point_cloud.push(Point{x: r*(theta0+3.*dth).cos(), y: r*(theta0+3.*dth).sin()});
	};
	assert_eq!(point_cloud.cover_circle().radius(), 1.);
	assert_eq!(point_cloud.len(), 7);

	// Remove original polygon vertices
	point_cloud.pop();
	point_cloud.pop();
	point_cloud.pop();
	assert_eq!(point_cloud.cover_circle().radius(), 0.5);
	assert_eq!(point_cloud.len(), 4);


	// Add new polygon vertices, w/o changing radius
	point_cloud.push(Point{x: 0.3, y: 0.3});
	assert_eq!(point_cloud.cover_circle().radius(), 0.5);
	assert_eq!(point_cloud.len(), 5);

	// Remove all but one point
	point_cloud.pop();
	point_cloud.pop();
	point_cloud.pop();
	point_cloud.pop();
	println!("{:?}", point_cloud.len());
	//println!("{}", point_cloud.cover_circle());
	//println!("{}", point_cloud.cover_circle().radius());
	assert_eq!(point_cloud.cover_circle().radius(), 0.);
	assert_eq!(point_cloud.len(), 1);
}
