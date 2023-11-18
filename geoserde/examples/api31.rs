use geo_types::Point;
use serde::Serialize;

fn main() {
    let _layer = [MyFeature::default()];
}

#[derive(Default, Serialize)]
struct MyFeature {
    #[serde(skip)] // terrible
    loc: Point,
    meter: i32,
}
impl geoserde::AsGeometry for MyFeature {
    fn as_geometry(&self) -> geoserde::GeometryRef {
        // self.loc.into()
        geoserde::GeometryRef {}
    }
}

mod geoserde {
    pub struct GeometryRef {}
    pub trait AsGeometry {
        fn as_geometry(&self) -> GeometryRef;
    }
}
