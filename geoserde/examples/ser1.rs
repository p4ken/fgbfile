use geo_types::{
    Coord, Geometry, GeometryCollection, LineString, MultiLineString, MultiPoint, Point, Polygon,
};
use geoserde::ser::FeatureSerializer;
use serde::Serialize;

fn main() {
    let mut ser = FeatureSerializer::new();

    let point = Point::new(11., 12.);
    let attribute = 1.;
    let point_layer = [PointFeature {
        geometry: point,
        attribute,
    }];
    point_layer[0].serialize(&mut ser).ok();

    let coords_layer = [CoordsFeature {
        geometry: vec![point.0],
        attribute,
    }];
    coords_layer[0].serialize(&mut ser).ok();

    let multi_point_layer = [MultiPointFeature {
        geometry: MultiPoint(vec![point]),
        attribute,
    }];
    multi_point_layer[0].serialize(&mut ser).ok();

    let line_string = LineString::from(vec![point]);
    let line_string_layer = [LineStringFeature {
        geometry: line_string.clone(),
        attribute,
    }];
    line_string_layer[0].serialize(&mut ser).ok();

    let multi_line_string_layer = [MultiLineStringFeature {
        geometry: MultiLineString(vec![line_string.clone()]),
        attribute,
    }];
    multi_line_string_layer[0].serialize(&mut ser).ok();

    let polygon_layer = [PolygonFeature {
        geometry: Polygon::new(line_string.clone(), vec![]),
        attribute,
    }];
    polygon_layer[0].serialize(&mut ser).ok();

    let geometry_layer = [GeometryFeature {
        geometry: line_string.clone().into(),
        attribute,
    }];
    geometry_layer[0].serialize(&mut ser).ok();

    let geometry_collection_layer = [GeometryCollectionFeature {
        geometry: vec![line_string.clone()].into(),
        attribute,
    }];
    geometry_collection_layer[0].serialize(&mut ser).ok();
}

#[derive(Serialize)]
struct PointFeature {
    geometry: Point,
    attribute: f64,
}

#[derive(Serialize)]
struct CoordsFeature {
    geometry: Vec<Coord>,
    attribute: f64,
}

#[derive(Serialize)]
struct MultiPointFeature {
    geometry: MultiPoint,
    attribute: f64,
}

#[derive(Serialize)]
struct LineStringFeature {
    geometry: LineString,
    attribute: f64,
}

#[derive(Serialize)]
struct PolygonFeature {
    geometry: Polygon,
    attribute: f64,
}

#[derive(Serialize)]
struct MultiLineStringFeature {
    geometry: MultiLineString,
    attribute: f64,
}

#[derive(Serialize)]
struct GeometryFeature {
    geometry: Geometry,
    attribute: f64,
}

#[derive(Serialize)]
struct GeometryCollectionFeature {
    geometry: GeometryCollection,
    attribute: f64,
}
