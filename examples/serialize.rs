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

    FgbFile::create("./layer.fgb")?.write_features(&my_layer)?;
    Ok(())
}

// The feature implements serde::Serialize
#[derive(Serialize)]
struct MyFeature {
    // The first geo-types field in the struct is the geometry.
    // The feature must have a geometry.
    road: LineString,

    // Rest of the fields are the prooerties.
    rank: i32,
}
