use std::vec;

use geo_types::{LineString, Polygon};
use serde::Serialize;

#[cfg(feature = "geozero")]
#[test]
fn geometry_test() -> anyhow::Result<()> {
    use geoserde::ser::GeometrySerializer;

    let mut buf = Vec::<u8>::new();
    let mut sink = geozero::wkt::WktWriter::new(&mut buf);
    let mut sut = GeometrySerializer::new(&mut sink);
    my_polygon().serialize(&mut sut)?;
    assert_eq!("POLYGON((0 0,1 1,1 0,0 0))", String::from_utf8(buf)?);
    Ok(())
}

fn my_polygon() -> Polygon {
    Polygon::new(
        LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.)]),
        vec![],
    )
}
