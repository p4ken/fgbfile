use fgbfile::ser::LayerSink;
use geo_types::Point;
use serde::Serialize;

// todo
fn main() {
    let layer = [Feat1 {
        geometry: Point::new(11, 12),
        number: 1,
    }];
    let mut format = LayerSink::new();
    layer[0].serialize(&mut format).ok();
}

#[derive(Serialize)]
struct Feat1 {
    geometry: Point<i32>,
    number: i32,
}
