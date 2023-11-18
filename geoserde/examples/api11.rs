use geo_types::{LineString, Point};
use serde::Serialize;

fn main() {
    let _layer = [Feature1::default()];
    geoserde::Serializer::geometry_name("loc"); // unrefactable!!
}

#[derive(Default, Serialize)]
struct Feature1 {
    id: i32,
    is_highway: bool,
    loc: Point,
}

struct Feature2 {
    shape: LineString,
    is_highway: bool,
}

mod geoserde {
    pub struct Serializer;
    impl Serializer {
        pub fn geometry_name(name: &str) {}
    }
}
