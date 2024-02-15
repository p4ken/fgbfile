# fgbfile

| [crates.io](https://crates.io/crates/fgbfile) 
| [docs.rs](https://docs.rs/fgbfile/latest/fgbfile/) 
| [github](https://github.com/p4ken/fgbfile) |

Simple builder and serializer for fgb files with wrapping [official flatgeobuf crate](https://crates.io/crates/flatgeobuf).

At this time only writing fgb is supported (not reading).

## Getting started

Add the dependency.

```sh
cargo add fgbfile
```

## Examples

Create my_layer.fgb and write two features.

```no_run
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
```

## License

[MIT license](LICENSE.txt)
