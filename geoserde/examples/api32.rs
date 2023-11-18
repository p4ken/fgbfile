use geo_types::Point;
use serde::Serialize;

fn main() {
    let _layer = [MyFeature::default()];
}

#[derive(Default, Serialize)]
struct MyFeature {
    // #[geoserde::geometry] // terrible
    loc: Point,
    meter: i32,
}
