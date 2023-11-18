use geo_types::{LineString, Point};
use serde::Serialize;

fn main() {}

#[derive(Default, Serialize)]
struct Feature1 {
    loc: Point, // first field is the geometry.
    id: i32,
    is_highway: bool,
}

struct Feature2 {
    shape: LineString, // first field is the geometry.
    is_highway: bool,
}
