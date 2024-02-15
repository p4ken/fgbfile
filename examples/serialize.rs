use fgbfile::FgbFile;
use geo_types::LineString;
use serde_derive::Serialize;

fn main() -> anyhow::Result<()> {
    let layer = [
        Feature {
            road: vec![(11., 21.)].into(),
            rank: 1,
        },
        Feature {
            road: vec![(12., 22.)].into(),
            rank: 2,
        },
    ];

    FgbFile::create("./my_layer.fgb")?.write_features(&layer)?;
    Ok(())
}

#[derive(Serialize)]
struct Feature {
    // geometry (the first geo_types field in the struct)
    road: LineString,

    // properties (any fields other than geometry)
    rank: i32,
}
