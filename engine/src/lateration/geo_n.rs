use crate::device;
use crate::utils::{Trace, Timestamp, DevId, Coords};
use crate::measure;
use ndarray::*;
use ndarray_linalg::Norm;


// protected List<Point> getIntersections(device_t d1, double dist1, device_t d2, double dist2)
// {
// 	List<Point> res = new List<Point>();
// 	double dx = d2.x - d1.x;
// 	double dy = d2.y - d1.y;
// 	double L2 = dx * dx + dy * dy;
// 	double rsum = (dist1 + dist2);
// 	double rdiff = (dist1 - dist2); // kolejnosc jest istotna
// 	double x, y;
// 	double lenI = 1 / Math.Sqrt(L2); // dx*lenI to wersor kierunku
// 	// rozlaczne wewnetrznie
// 	if (L2 <= rdiff*rdiff)
// 	{
// 		// rozlaczne wewnetrznie
// 		x = (d1.x + d2.x - dx * lenI * rsum) / 2;
// 		y = (d1.y + d2.y - dy * lenI * rsum) / 2;
// 		res.Add(new Point((int)x, (int)y));
// 		res.Add(new Point((int)x, (int)y));
// 	}
// 	else if(rsum*rsum < L2)
// 	{

// 		// rozlaczne zewnetrznie
// 		x = (d1.x + d2.x + dx * lenI * rdiff) / 2;
// 		y = (d1.y + d2.y + dy * lenI * rdiff) / 2;
// 		res.Add(new Point((int)x, (int)y));
// 		res.Add(new Point((int)x, (int)y));
// 	}
// 	// gdy odleglosci sie przecinaja z pewnym zapasem
// 	else
// 	{
// 		double kk = (rsum*rsum - L2) * (L2 - rdiff*rdiff);
// 		double K = Math.Sqrt(kk) / 4; // pole trojkata
// 		x = (int)(d1.x + d2.x + (d2.x-d1.x)*(dist1 * dist1 - dist2 * dist2) /L2) / 2;
// 		y = (int)(d1.y + d2.y + (d2.y-d1.y)*(dist1 * dist1 - dist2 * dist2) /L2) / 2;
// 		res.Add(new Point((int)(x + 2 * (d2.y - d1.y) * K / L2),
// 							(int)(y - 2 * (d2.x - d1.x) * K / L2)));
// 		res.Add(new Point((int)(x - 2 * (d2.y - d1.y) * K / L2),
// 							(int)(y + 2 * (d2.x - d1.x) * K / L2)));
// 	}
// 	return res;
// }

struct CrossPoints {
	points : Vec<Coords>,
}

impl CrossPoints {
	fn Create() -> CrossPoints {
		CrossPoints {
			points: Vec::new(),
		}
	}

	fn add_cross_2d(&mut self, p1 : &Coords, r1: f32, p2: &Coords, r2: f32)
	{
		let mut result : Vec<Coords> = Vec::new();
		let dx = p2[0] - p1[0];
		let dy = p2[1] - p1[1];
		let L2 = dx * dx + dy * dy;
		let rsum = r1 + r2;
		let rdiff = r2 - r1;
		let lenI = 1.0 / L2.sqrt(); // dx*lenI to wersor kierunku
		if L2 <= rdiff * rdiff
		{
			// rozlaczne wewnetrznie
			let x = (p1[0] + p2[0] - dx * lenI * rsum) / 2.0;
			let y = (p1[1] + p2[1] - dy * lenI * rsum) / 2.0;
			self.points.push(Coords([x, y, 0.0]));
		}
		else if rsum*rsum < L2
		{
			// rozlaczne zewnetrznie
			let x = (p1[0] + p2[0] - dx * lenI * rdiff) / 2.0;
			let y = (p1[1] + p2[1] - dy * lenI * rdiff) / 2.0;
			self.points.push(Coords([x, y, 0.0]));
		}
		else
		{
			// gdy odleglosci sie przecinaja z pewnym zapasem
			let kk = (rsum*rsum - L2) * (L2 - rdiff*rdiff);
			let K = kk.sqrt() / 4.0; // pole trojkata
			let t = (r1 * r1 - r2 * r2) / L2;
			let x = (p1[0] + p2[0] + dx * t) / 2.0;
			let y = (p1[1] + p2[1] + dy * t) / 2.0;
			self.points.push(Coords([x + 2.0 * dy * K / L2, y - 2.0 * dx * K / L2, 0.0]));
			self.points.push(Coords([x - 2.0 * dy * K / L2, y + 2.0 * dx * K / L2, 0.0]))
		}
	}

	fn centroid(self) -> Coords
	{
		let result = Coords([0., 0., 0.]);
		let sum = 0.0;

		for cross in self.points {
			result += cross;
			sum += 1.0;
		}

		result /= sum;
		result
	}
}

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