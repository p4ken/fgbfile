use geo_types::LineString;
use geoserde::ser::FeatureSerializer;
use serde::Serialize;

#[cfg(feature = "geozero")]
#[test]
fn serialize_test() {
    let mut sink = geozero::ProcessorSink;
    let mut sut = FeatureSerializer::new(&mut sink);

    let line_string_layer = [LineStringFeature {
        geometry: vec![(139.691667, 35.689722), (139.7454329, 35.6585805)].into(),
        attribute: 1.0,
    }];
    line_string_layer[0].serialize(&mut sut).unwrap();
}

#[derive(Serialize)]
struct LineStringFeature {
    geometry: LineString,
    attribute: f64,
}
