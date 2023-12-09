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
impl Container {
    fn as_str(&self) -> &'static str {
        use Container::*;
        match self {
            Coord => "Coord",
            Point => "Point",
            MultiPoint => "MultiPoint",
            Line => "Line",
            LineString => "LineString",
            MultiLineString => "MultiLineString",
            Polygon => "Polygon",
            MultiPolygon => "MultiPolygon",
            Geometry => "Geometry",
            GeometryCollection => "GeometryCollection",
        }
    }
}

// pub trait GeometrySink: geozero::GeomProcessor {}
// impl<G: geozero::GeomProcessor> GeometrySink for G {}
pub trait GeometrySink {
    type Error: std::error::Error;
    fn xy(&mut self, x: f64, y: f64, index: usize) -> Result<(), Self::Error>;
    // fn point_begin(&mut self, index: usize) -> Result<(), Self::Error>;
    // fn point_end(&mut self, index: usize) -> Result<(), Self::Error>;
    fn linestring_begin(
        &mut self,
        is_single: bool,
        len: usize,
        index: usize,
    ) -> Result<(), Self::Error>;
    fn linestring_end(&mut self, is_single: bool, index: usize) -> Result<(), Self::Error>;
    // fn geometry_begin(&mut self) -> Result<(), Self::Error>;
    // fn geometry_end(&mut self) -> Result<(), Self::Error>;
}
#[cfg(feature = "geozero")]
impl<G: geozero::GeomProcessor> GeometrySink for G {
    type Error = geozero::error::GeozeroError;
    fn xy(&mut self, x: f64, y: f64, index: usize) -> Result<(), Self::Error> {
        self.xy(x, y, index)
    }
    // fn point_begin(&mut self, index: usize) -> Result<(), Self::Error> {
    //     self.point_begin(index)
    // }
    // fn point_end(&mut self, index: usize) -> Result<(), Self::Error> {
    //     self.point_end(index)
    // }
    fn linestring_begin(
        &mut self,
        is_single: bool,
        len: usize,
        index: usize,
    ) -> Result<(), Self::Error> {
        self.linestring_begin(is_single, len, index)
    }
    fn linestring_end(&mut self, is_single: bool, index: usize) -> Result<(), Self::Error> {
        self.linestring_end(is_single, index)
    }
    // fn geometry_begin(&mut self) -> Result<(), Self::Error> {
    //     self.geometry_begin()
    // }
    // fn geometry_end(&mut self) -> Result<(), Self::Error> {
    //     self.geometry_end()
    // }
}

pub struct GeometrySerializer<'a, S> {
    /// May have to cache geometry type.
    stack: Vec<Container>,
    x: Option<f64>,
    coord_index: usize,
    line_index: usize,

    sink: &'a mut S,
}

impl<'a, S> GeometrySerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            stack: vec![],
            x: None,
            coord_index: 0,
            line_index: 0,
            sink,
        }
    }
}

impl<S: GeometrySink> Serializer for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;
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
        self.serialize_f64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match self.stack.last() {
            Some(Container::Coord) => (),
            arm => {
                return Err(SerializeError::InvalidGeometryContainer {
                    name: match arm {
                        Some(container) => container.as_str(),
                        None => "None",
                    },
                    expected: "Coord",
                })
            }
        }
        dbg!(v);
        match self.x {
            Some(x) => {
                dbg!(self.coord_index);
                self.sink
                    .xy(x, v, self.coord_index)
                    .map_err(SerializeError::GeometrySinkCaused)?;
                self.x = None;
                self.coord_index += 1;
            }
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
        let container = match name {
            "LineString" => Container::LineString,
            name => {
                return Err(SerializeError::InvalidGeometryContainer {
                    name,
                    expected: "LineString",
                })
            }
        };
        self.stack.push(container);
        value.serialize(&mut *self)?;
        dbg!();
        Ok(())
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
        let len = len.ok_or(SerializeError::InvalidGeometryContainer {
            name: "?",
            expected: "Vec",
        })?;
        dbg!(len);
        match self.stack.last() {
            Some(Container::LineString) => {
                // if self.line_index == 0 {
                //     self.sink
                //         .geometry_begin()
                //         .map_err(SerializeError::GeometrySinkCaused)?;
                // }
                self.sink
                    .linestring_begin(true, len, self.line_index)
                    .map_err(SerializeError::GeometrySinkCaused)?;
            }
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
        let container = match name {
            "Coord" => Container::Coord,
            name => {
                return Err(SerializeError::InvalidGeometryContainer {
                    name,
                    expected: "Coord",
                })
            }
        };
        self.stack.push(container);
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

impl<S: GeometrySink> SerializeSeq for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dbg!();
        match self.stack.pop() {
            Some(Container::LineString) => {
                self.sink
                    .linestring_end(true, self.line_index)
                    .map_err(SerializeError::GeometrySinkCaused)?;
                self.coord_index = 0;
                self.line_index += 1;
            }
            Some(_) => todo!(),
            None => (),
        }
        // if self.stack.is_empty() {
        //     self.sink
        //         .geometry_end()
        //         .map_err(SerializeError::GeometrySinkCaused)?;
        // }
        Ok(())
    }
}

impl<S: GeometrySink> SerializeTuple for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

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

impl<S: GeometrySink> SerializeTupleStruct for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

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

impl<S: GeometrySink> SerializeTupleVariant for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

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

impl<S: GeometrySink> SerializeMap for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

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

impl<S: GeometrySink> SerializeStruct for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        dbg!(key);
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dbg!();
        self.stack.pop();
        Ok(())
    }
}

impl<S: GeometrySink> SerializeStructVariant for &mut GeometrySerializer<'_, S> {
    type Ok = ();
    type Error = SerializeError<S::Error>;

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
