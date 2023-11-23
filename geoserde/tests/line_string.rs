use geo_types::LineString;
use serde::Serialize;

#[cfg(feature = "geozero")]
#[test]
fn geometry_test() {
    use geoserde::ser::GeometrySerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);

    let line_string = LineString::from(vec![(139.691667, 35.689722), (139.7454329, 35.6585805)]);
    line_string.serialize(&mut sut).unwrap();
    assert_eq!(
        "LINESTRING(139.691667 35.689722,139.7454329 35.6585805)",
        String::from_utf8(buf).unwrap()
    );
}

#[cfg(feature = "geozero")]
#[test]
fn feature_test() {
    use geoserde::ser::FeatureSerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::geojson::GeoJsonWriter::new(&mut buf);
    let mut sut = FeatureSerializer::new(&mut sink);

    let line_string_layer = [LineStringFeature {
        geometry: vec![(139.691667, 35.689722), (139.7454329, 35.6585805)].into(),
        attribute: 1.0,
    }];
    line_string_layer[0].serialize(&mut sut).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
}

#[derive(Serialize)]
struct LineStringFeature {
    geometry: LineString,
    attribute: f64,
}
