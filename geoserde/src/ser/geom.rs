use std::str::FromStr;

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
impl FromStr for Container {
    type Err = SerializeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Container::*;
        let container = match s {
            "Coord" => Coord,
            "LineString" => LineString,
            _ => return Err(SerializeError::NotAGeometryField("")),
        };
        Ok(container)
    }
}

// pub trait GeometrySink: geozero::GeomProcessor {}
// impl<G: geozero::GeomProcessor> GeometrySink for G {}
pub trait GeometrySink {
    fn xy(&mut self, x: f64, y: f64) -> Result<(), SerializeError>;
    fn linestring_begin(&mut self, is_single: bool, len: usize) -> Result<(), SerializeError>;
    fn linestring_end(&mut self, is_single: bool) -> Result<(), SerializeError>;
    // fn point_begin(&mut self, index: usize) -> Result<(), SerializeError>;
    // fn point_end(&mut self, index: usize) -> Result<(), SerializeError>;
}
#[cfg(feature = "geozero")]
impl<G: geozero::GeomProcessor> GeometrySink for G {
    fn xy(&mut self, x: f64, y: f64) -> Result<(), SerializeError> {
        self.xy(x, y, 0)?;
        Ok(())
    }
    fn linestring_begin(&mut self, is_single: bool, len: usize) -> Result<(), SerializeError> {
        self.linestring_begin(is_single, len, 0)?;
        Ok(())
    }
    fn linestring_end(&mut self, is_single: bool) -> Result<(), SerializeError> {
        self.linestring_end(is_single, 0)?;
        Ok(())
    }
    // fn point_begin(&mut self, index: usize) -> Result<(), SerializeError> {
    //     self.point_begin(index)?;
    //     Ok(())
    // }
    // fn point_end(&mut self, index: usize) -> Result<(), SerializeError> {
    //     self.point_end(index)?;
    //     Ok(())
    // }
}

pub struct GeometrySerializer<'a, S> {
    /// May have to cache geometry type.
    stack: Vec<Container>,
    x: Option<f64>,
    sink: &'a mut S,
}

impl<'a, S> GeometrySerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            stack: vec![],
            x: None,
            sink,
        }
    }
}

impl<S: GeometrySink> Serializer for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError;
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
        match self.x {
            Some(x) => self.sink.xy(x, v)?,
            None => self.x = Some(v),
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
        dbg!(name);
        self.stack.push(name.parse()?);
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
        let len = len.ok_or(SerializeError::NotAGeometryField(""))?;
        dbg!(len);
        match self.stack.last() {
            Some(Container::LineString) => self.sink.linestring_begin(true, len)?,
            _ => (), // todo
        }
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
        self.stack.push(name.parse()?);
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
        match self.stack.pop() {
            Some(Container::LineString) => self.sink.linestring_end(true)?,
            Some(_) => todo!(),
            None => (),
        }
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
        self.stack.pop();
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
