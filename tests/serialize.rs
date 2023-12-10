use std::io::Cursor;

use fgbfile::FgbFile;
use flatgeobuf::{geozero::ToGeo, FallibleStreamingIterator, FeatureProperties, FgbReader};
use geo_types::LineString;
use serde_derive::Serialize;

#[test]
fn serialize_to_fgb() -> anyhow::Result<()> {
    let mut buf = vec![];
    let layer = [Feature {
        shape: vec![(10., 20.)].into(),
        rank: 1,
    }];
    let count = FgbFile::new(&mut buf, "my_layer").write_features(&layer)?;
    assert_eq!(count, 1);

    let cursor = Cursor::new(buf);
    let mut fgb_iter = FgbReader::open(cursor)?.select_all()?;
    let feat = fgb_iter.next()?.unwrap();
    assert_eq!(feat.property::<i32>("rank")?, layer[0].rank);
    assert_eq!(LineString::try_from(feat.to_geo()?)?, layer[0].shape);
    Ok(())
}

#[derive(Serialize)]
struct Feature {
    shape: LineString,
    rank: i32,
}
