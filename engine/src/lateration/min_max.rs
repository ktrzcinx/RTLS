pub fn calc_dev_position(dev: &device::Data, measures: &measure::DistTable,
	devices: &Vec<&device::Data>, timestamp: Timestamp) -> Trace
{
	let ip: CrossPoints = CrossPoints::Create();
	for i in 1..devices.len() {
		let id1 = devices[i].id();
		let p1 = devices[i].estimate_position(timestamp).coords;
		let r1 = measures.estimate(id1, dev.id(), timestamp);
		for j in 0..i-1 {
			let id2 = devices[j].id();
			let p2 = devices[j].estimate_position(timestamp).coords;
			let r2 = measures.estimate(id2, dev.id(), timestamp);
			ip.add_cross_2d(&p1, r1, &p2, r2);
		}
	}
	Trace {
		coords: ip.centroid(),
		timestamp: timestamp,
	}
}