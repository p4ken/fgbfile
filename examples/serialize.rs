use fgbfile::FgbFile;
use geo_types::LineString;
use serde_derive::Serialize;

fn main() -> anyhow::Result<()> {
    let my_layer = [
        MyFeature {
            road: vec![(11., 21.)].into(),
            rank: 1,
        },
        MyFeature {
            road: vec![(12., 22.)].into(),
            rank: 2,
        },
    ];

    let mut fgb = FgbFile::create("./layer.fgb")?;
    for feat in my_layer {
        fgb.deserialize_property(&feat);
        fgb.write_geometry(&feat.road);
    }
    Ok(())
}

// The feature implements serde::Serialize
#[derive(Serialize)]
struct MyFeature {
    #[serde(skip)]
    road: LineString,

    // Rest of the fields are the prooerties.
    rank: i32,
}
