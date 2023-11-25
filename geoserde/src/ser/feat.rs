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
    sink: &'a mut S,
    has_geom: bool,
    geom_key: &'static str,
    prop_index: usize,
}

impl<'a, S: FeatureSink> FeatureSerializer<'a, S> {
    pub fn new(sink: &'a mut S) -> Self {
        Self {
            sink,
            has_geom: false,
            geom_key: "geometry",
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
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        // flatten it
        value.serialize(self)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // field key is required
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        // field key is required
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        // field key is required
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // field key is required
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // field key is required
        Err(SerializeError::MalformedFeature)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

impl<'a, S: FeatureSink> SerializeSeq for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

impl<'a, S: FeatureSink> SerializeTuple for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}
impl<'a, S: FeatureSink> SerializeTupleStruct for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

impl<'a, S: FeatureSink> SerializeTupleVariant for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}
impl<'a, S: FeatureSink> SerializeMap for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
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
            Err(SerializeError::NoGeometryField)
        }
    }
}

impl<'a, S: FeatureSink> SerializeStructVariant for &mut FeatureSerializer<'a, S> {
    type Ok = ();
    type Error = SerializeError<<S as GeometrySink>::Error>;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}
