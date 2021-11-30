mod min_max;
mod geo_n;

pub trait Lateration {
    pub fn calc_dev_position(dev: &device::Data, measures: &measure::DistTable,
        devices: &Vec<&device::Data>, timestamp: Timestamp) -> Trace
}

#[derive(Debug)]
pub enum LaterationTypes {
    MinMax(min_max),
    GeoN
}

pub struct LaterationFactory {}

impl LaterationFactory {
    pub fn get(algorithm: &str) -> Shape {
        match shape.as_ref() {
            "CIRCLE" => Shape::Circle(Circle {}),
            "SQUARE" => Shape::Square(Square {}),
            "RECTANGLE" => Shape::Rectangle(Rectangle {}),
            &_ => unimplemented!(),
        }
    }
}