use smallest_circle::smallest_cover;

#[test]
fn point_cloud_use() {
	let mut point_cloud = smallest_cover::PointCloud::new();

	point_cloud.extend(0u8..5u8);
	assert_eq!(point_cloud.cover_radius(), 0.);  // replace w/ calculated value

	for i in 5u8..10u8 {
		point_cloud.insert(i);
		point_cloud.remove(i-5);
		assert_eq!(point_cloud.cover_radius(), 0.);  // replace w/ calculated value
	}
}
