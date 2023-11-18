use geo_types::Point;
use serde::Serialize;

fn main() {
    let layer = &[Feature1::default()];
    geoserde::serialize(
        layer,
        |feat| feat.is_highway,
        |feat| (feat.id, feat.is_highway),
    );
}

#[derive(Default, Serialize)]
struct Feature1 {
    id: i32,
    is_highway: bool,
    loc: Point,
}

mod geoserde {
    pub fn serialize<T, G, P>(_layer: &[T], _geom: impl Fn(T) -> G, _prop: impl Fn(T) -> P) {
        // how to process P ??
    }
}
