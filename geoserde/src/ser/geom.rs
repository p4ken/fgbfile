use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use super::SerializeError;

enum Container {
    Coord,
    Point,
    MultiPoint,
    Line,
    LineString,
    MultiLineString,
    Polygon,
    MultiPolygon,
    Geometry,
    GeometryCollection,
}

pub trait GeometrySink {
    type Error: std::error::Error;
    fn xy(&mut self, x: f64, y: f64, index: usize) -> Result<(), Self::Error>;
}
// #[cfg(feature = "fgb")]
// impl GeometrySink for flatgeobuf::FgbWriter<'_> {
//     type Error = flatgeobuf::geozero::error::GeozeroError;

//     fn xy(&mut self, x: f64, y: f64, idx: usize) -> Result<(), Self::Error> {
//         self.xy(x, y, idx)
//     }
// }
#[cfg(feature = "geozero")]
impl<G: geozero::GeomProcessor> GeometrySink for G {
    type Error = geozero::error::GeozeroError;

    fn xy(&mut self, x: f64, y: f64, index: usize) -> Result<(), Self::Error> {
        self.xy(x, y, index)
    }
}
// impl<G: GeometrySink> GeometrySink for &mut G {
//     type Error = G::Error;

//     fn xy(&mut self, x: f64, y: f64, index: usize) -> Result<(), Self::Error> {
//         self.xy(x, y, index)
//     }
// }

pub struct GeometrySerializer<'a, S> {
    /// enumのGeometryなら必要だがそれ以外なら要らない
    /// enumのGeometryなら直接geozeroを呼べる
    /// May have to cache geometry type.
    stack: Vec<Container>,
    x: Option<f64>,
    i_coord: usize,

    sink: &'a mut S,
}

impl<'a, S: GeometrySink> GeometrySerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            stack: vec![],
            x: None,
            i_coord: 0,
            sink,
        }
    }
}

impl<'a, S: GeometrySink> Serializer for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        dbg!(v);
        if let Some(x) = self.x {
            self.sink.xy(x, v, self.i_coord);
        }
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // TODO: convert name to enum
        dbg!(name);
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        dbg!(name);
        dbg!(variant_index);
        dbg!(variant);
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        dbg!(len);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        dbg!(name);
        dbg!(len);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a, S: GeometrySink> SerializeSeq for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dbg!();
        Ok(())
    }
}

impl<'a, S: GeometrySink> SerializeTuple for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, S: GeometrySink> SerializeTupleStruct for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, S: GeometrySink> SerializeTupleVariant for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, S: GeometrySink> SerializeMap for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, S: GeometrySink> SerializeStruct for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        dbg!(key);
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dbg!();
        Ok(())
    }
}

impl<'a, S: GeometrySink> SerializeStructVariant for &mut GeometrySerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use geo_types::LineString;

//     use super::*;

//     #[cfg(feature = "geozero")]
//     #[test]
//     fn line_string_test() {
//         let geom = LineString::from(vec![(1.0, 2.0)]);
//         let mut sink = geozero::GeozeroGeometry;
//         let mut sut = GeometrySerializer::new(&mut sink);
//         geom.serialize(&mut sut).unwrap();
//     }
// }
