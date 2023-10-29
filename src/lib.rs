mod err;
pub mod ser;

use std::{
    borrow::Cow,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use flatgeobuf::{
    geozero::{
        error::GeozeroError, ColumnValue, FeatureProcessor, GeomProcessor, PropertyProcessor,
    },
    FgbWriter, FgbWriterOptions, GeometryType, GeozeroGeometry,
};

pub use err::FgbFileError;
pub use ser::feat::FeatureSerializer; // tmp

/// Output feature
pub trait ToFgb<const N: usize> {
    fn geometry(&self) -> FgbGeometry;
    fn properties(&self) -> [FgbProperty; N];
}

impl<T: ToFgb<N>, const N: usize> ToFgb<N> for &T {
    fn geometry(&self) -> FgbGeometry {
        (*self).geometry()
    }

    fn properties(&self) -> [FgbProperty; N] {
        (*self).properties()
    }
}

/// Output feature geometry
#[derive(Debug)]
pub enum FgbGeometry<'a> {
    Point(Cow<'a, geo_types::Point>),
    LineString(&'a geo_types::LineString),
}

impl<'a> From<&'a geo_types::Point> for FgbGeometry<'a> {
    fn from(geo: &'a geo_types::Point) -> Self {
        Self::Point(Cow::Borrowed(geo))
    }
}

impl<'a> From<&'a geo_types::LineString> for FgbGeometry<'a> {
    fn from(geo: &'a geo_types::LineString) -> Self {
        Self::LineString(geo)
    }
}

/// Output feature property
pub struct FgbProperty<'a> {
    name: &'a str,
    value: FgbValue<'a>,
}

impl<'a, T: Into<FgbValue<'a>>> From<(&'a str, T)> for FgbProperty<'a> {
    fn from((name, value): (&'a str, T)) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }
}

/// Output property value
pub struct FgbValue<'a>(ColumnValue<'a>);

impl From<bool> for FgbValue<'_> {
    fn from(primitive: bool) -> Self {
        Self(ColumnValue::Bool(primitive))
    }
}

impl From<i8> for FgbValue<'_> {
    fn from(primitive: i8) -> Self {
        Self(ColumnValue::Byte(primitive))
    }
}

impl From<u8> for FgbValue<'_> {
    fn from(primitive: u8) -> Self {
        Self(ColumnValue::UByte(primitive))
    }
}

impl From<i16> for FgbValue<'_> {
    fn from(primitive: i16) -> Self {
        Self(ColumnValue::Short(primitive))
    }
}

impl From<u16> for FgbValue<'_> {
    fn from(primitive: u16) -> Self {
        Self(ColumnValue::UShort(primitive))
    }
}

impl From<i32> for FgbValue<'_> {
    fn from(primitive: i32) -> Self {
        Self(ColumnValue::Int(primitive))
    }
}

impl From<u32> for FgbValue<'_> {
    fn from(primitive: u32) -> Self {
        Self(ColumnValue::UInt(primitive))
    }
}

impl From<i64> for FgbValue<'_> {
    fn from(primitive: i64) -> Self {
        Self(ColumnValue::Long(primitive))
    }
}

impl From<u64> for FgbValue<'_> {
    fn from(primitive: u64) -> Self {
        Self(ColumnValue::ULong(primitive))
    }
}

impl From<f32> for FgbValue<'_> {
    fn from(primitive: f32) -> Self {
        Self(ColumnValue::Float(primitive))
    }
}

impl From<f64> for FgbValue<'_> {
    fn from(primitive: f64) -> Self {
        Self(ColumnValue::Double(primitive))
    }
}

impl<'a> From<&'a str> for FgbValue<'a> {
    fn from(primitive: &'a str) -> Self {
        Self(ColumnValue::String(primitive))
    }
}

impl<'a> From<&'a String> for FgbValue<'a> {
    fn from(primitive: &'a String) -> Self {
        Self(ColumnValue::String(primitive))
    }
}

impl GeozeroGeometry for FgbGeometry<'_> {
    fn process_geom<P: GeomProcessor>(&self, processor: &mut P) -> Result<(), GeozeroError> {
        match self {
            FgbGeometry::Point(point) => {
                processor.point_begin(0)?;
                processor.xy(point.x(), point.y(), 0)?;
                processor.point_end(0)?;
            }
            FgbGeometry::LineString(line_string) => {
                processor.linestring_begin(true, line_string.0.len(), 0)?;
                for (i, coord) in line_string.coords().enumerate() {
                    processor.xy(coord.x, coord.y, i)?;
                }
                processor.linestring_end(true, 0)?;
            }
        }
        Ok(())
    }
}

pub struct FgbFile<'a, B> {
    buf: B,
    name: Cow<'a, str>,
    options: FgbWriterOptions<'static>,
}

impl<'a> FgbFile<'a, BufWriter<File>> {
    /// Create a new FlatGeobuf file in the local path.
    pub fn create(path: &'a impl AsRef<Path>) -> std::io::Result<Self> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let buf = BufWriter::new(File::create(path)?);
        let name = path
            .as_ref()
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        Ok(Self::new(buf, name))
    }
}

impl<'a, W: Write> FgbFile<'a, W> {
    fn new(buf: W, name: Cow<'a, str>) -> Self {
        Self {
            buf,
            name,
            options: FgbWriterOptions {
                promote_to_multi: false,
                ..Default::default()
            },
        }
    }

    /// Set EPSG code.
    pub fn epsg(mut self, epsg: i32) -> Self {
        self.options.crs.code = epsg;
        self
    }

    /// Write features to the fgb file.
    pub fn write_all<I, T, const N: usize>(self, features: I) -> Result<u64, GeozeroError>
    where
        I: IntoIterator<Item = T>,
        T: ToFgb<N>,
    {
        let mut writer =
            FgbWriter::create_with_options(&self.name, GeometryType::Unknown, self.options)?;

        let mut count = 0;
        for feat in features {
            feat.geometry().process_geom(&mut writer)?;
            for (i, prop) in feat.properties().iter().enumerate() {
                writer.property(i, prop.name, &prop.value.0)?;
            }
            writer.feature_end(0)?;
            count += 1;
        }

        writer.write(self.buf)?;
        Ok(count)
    }
}
