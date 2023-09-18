use geo_types::{LineString, Geometry};
use geozero::{GeozeroDatasource, GeozeroGeometry, ColumnValue};

pub fn main() {
    
}

struct Spot {
    id: u32,
    name: String,
    geom: LineString,
}

impl GeozeroDatasource for Spot {
    fn process<P: flatgeobuf::geozero::FeatureProcessor>(
        &mut self,
        processor: &mut P,
    ) -> flatgeobuf::geozero::error::Result<()> {
        Geometry::from(self.geom.clone()).process_geom(processor)?;
        processor.property(0, "id", &ColumnValue::UInt(self.id))?;
        todo!()
    }
}
