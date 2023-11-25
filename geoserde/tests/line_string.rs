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
fn property_test() -> anyhow::Result<()> {
    use geoserde::ser::PropertySerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::geojson::GeoJsonWriter::new(&mut buf);
    let mut sut = PropertySerializer::new(0, "my_property", &mut sink);
    my_property().serialize(&mut sut)?;
    assert_eq!(r#""double": 1.1, "integer": 2"#, String::from_utf8(buf)?);
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
fn my_property() -> MyProperty {
    MyProperty {
        double: 1.1,
        integer: 2,
    }
}
fn my_feature() -> MyFeature {
    let MyProperty { double, integer } = my_property();
    MyFeature {
        geometry: my_geometry(),
        double,
        integer,
    }
}
#[derive(Serialize)]
struct MyProperty {
    double: f64,
    integer: i32,
}
#[derive(Serialize)]
struct MyFeature {
    double: f64,
    geometry: LS,
    integer: i32,
}
