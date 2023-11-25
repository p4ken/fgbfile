use geo_types::LineString as LS;
use serde::Serialize;

#[cfg(feature = "geozero")]
#[test]
fn geometry_test() -> anyhow::Result<()> {
    use geoserde::ser::GeometrySerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    my_geometry().serialize(&mut sut)?;
    assert_eq!(
        "LINESTRING(139.691667 35.689722,139.7454329 35.6585805)",
        String::from_utf8(buf)?
    );
    Ok(())
}

#[cfg(feature = "geozero")]
#[test]
fn feature_test() -> anyhow::Result<()> {
    use geoserde::ser::FeatureSerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::geojson::GeoJsonWriter::new(&mut buf);
    let mut sut = FeatureSerializer::new(&mut sink);
    my_feature().serialize(&mut sut)?;
    println!("{}", String::from_utf8(buf)?);
    Ok(())
}

fn my_geometry() -> LS {
    vec![(139.691667, 35.689722), (139.7454329, 35.6585805)].into()
}
fn my_feature() -> MyFeature {
    MyFeature {
        geometry: my_geometry(),
        attribute: 1.0,
    }
}
#[derive(Serialize)]
struct MyFeature {
    attribute: f64,
    geometry: LS,
}
