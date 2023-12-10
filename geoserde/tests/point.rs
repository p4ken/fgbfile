// todo

use geo_types::Point;
use geoserde::ser::FeatureSerializer;
use serde::Serialize;

#[cfg(feature = "geozero")]
#[test]
fn geometry_test() -> anyhow::Result<()> {
    use geoserde::ser::GeometrySerializer;
    use geozero::wkt::WktWriter;

    let mut buf = Vec::<u8>::new();
    let mut sink = WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    my_point().serialize(&mut sut)?;
    assert_eq!("POINT(139.5860139 35.4813408)", String::from_utf8(buf)?);
    Ok(())
}

#[cfg(feature = "geozero")]
#[test]
fn feature_test() -> anyhow::Result<()> {
    use geozero::geojson::GeoJsonWriter;

    let feature = MyFeature {
        id: "SO51",
        pos: my_point(),
    };
    let mut buf = Vec::<u8>::new();
    let mut sink = GeoJsonWriter::new(&mut buf);
    let mut sut = FeatureSerializer::new(&mut sink);
    feature.serialize(&mut sut)?;
    println!("{}", String::from_utf8(buf)?);
    Ok(())
}

fn my_point() -> Point {
    Point::new(139.5860139, 35.4813408)
}

#[derive(Serialize)]
struct MyFeature {
    id: &'static str,
    pos: Point,
}
