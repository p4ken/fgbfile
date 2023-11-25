use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

use super::{GeometrySerializer, GeometrySink, PropertySerializer, PropertySink, SerializeError};

pub trait FeatureSink: GeometrySink + PropertySink<Error = <Self as GeometrySink>::Error> {}
impl<T: GeometrySink + PropertySink<Error = <Self as GeometrySink>::Error>> FeatureSink for T {}

pub struct FeatureSerializer<'a, S: FeatureSink> {
    // current_key: &'static str,
    sink: &'a mut S,
    has_geom: bool,
    geom_key: &'static str,
    // geom_error: Option<SerializeError<<S as GeometrySink>::Error>>,
    prop_index: usize,
}

impl<'a, S: FeatureSink> FeatureSerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            // current_key: "",
            sink,
            has_geom: false,
            geom_key: "geometry",
            // geom_error: None,
            prop_index: 0,
        }
    }
    pub fn geom_field(&mut self, key: &'static str) -> &Self {
        self.geom_key = key;
        self
    }
}

impl<'a, S: FeatureSink> Serializer for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    type SerializeSeq = Self;
    type SerializeTuple = Self::SerializeSeq;
    type SerializeTupleStruct = Self::SerializeSeq;
    type SerializeTupleVariant = Self::SerializeSeq;
    type SerializeMap = Self::SerializeSeq;
    type SerializeStruct = Self::SerializeSeq;
    type SerializeStructVariant = Self::SerializeSeq;

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
        todo!()
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
        todo!()
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
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
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

impl<'a, S: FeatureSink> SerializeSeq for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

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

impl<'a, S: FeatureSink> SerializeTuple for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

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
impl<'a, S: FeatureSink> SerializeTupleStruct for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

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

impl<'a, S: FeatureSink> SerializeTupleVariant for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

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
impl<'a, S: FeatureSink> SerializeMap for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

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

impl<'a, S: FeatureSink> SerializeStruct for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if !self.has_geom {
            // try to serialize as a geometry
            let mut geom = GeometrySerializer::new(self.sink);
            match value.serialize(&mut geom) {
                // found the first geometry field
                Ok(()) => {
                    self.has_geom = true;
                    return Ok(());
                }

                // the field must be a geometry
                Err(e) if key == self.geom_key => return Err(e),

                // ignore the error
                Err(_) => (),
            }
        }

        // serialize as a property
        let mut prop = PropertySerializer::new(self.prop_index, key, self.sink);
        self.prop_index = value.serialize(&mut prop)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.has_geom {
            Ok(())
        } else {
            Err(SerializeError::MissingGeometryField)
        }
    }
}

impl<'a, S: FeatureSink> SerializeStructVariant for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

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
