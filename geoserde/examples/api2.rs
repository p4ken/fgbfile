use geo_types::Point;
use serde::Serialize;

fn main() {
    let _layer = [MyFeature::default()];
    geoserde::Serializer::geometry_name("loc"); // unrefactable!!
}

#[derive(Default, Serialize)]
struct MyFeature {
    loc: Point,
    meter: i32,
}

mod geoserde {
    pub struct Serializer;
    impl Serializer {
        pub fn geometry_name(name: &str) {}
    }
}
