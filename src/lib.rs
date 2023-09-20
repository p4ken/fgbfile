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

pub trait ToFgb<const N: usize> {
    fn geometry(&self) -> FgbGeometry;
    fn properties(&self) -> [FgbProperty; N];
}

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

pub struct FgbValue<'a>(ColumnValue<'a>);

impl From<bool> for FgbValue<'_> {
    fn from(primitive: bool) -> Self {
        Self(ColumnValue::Bool(primitive))
    }
}

impl From<u32> for FgbValue<'_> {
    fn from(primitive: u32) -> Self {
        Self(ColumnValue::UInt(primitive))
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

pub struct FgbFile<'a, W> {
    buf: W,
    name: Cow<'a, str>,
    options: FgbWriterOptions<'static>,
}

impl<'a> FgbFile<'a, BufWriter<File>> {
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
    pub fn new(buf: W, name: Cow<'a, str>) -> Self {
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
    pub fn write_all<'b, I, T, const N: usize>(self, features: I) -> Result<(), GeozeroError>
    where
        I: IntoIterator<Item = &'b T>,
        T: ToFgb<N> + 'b,
    {
        let mut writer =
            FgbWriter::create_with_options(&self.name, GeometryType::Unknown, self.options)?;
        for feat in features {
            feat.geometry().process_geom(&mut writer)?;
            for (i, prop) in feat.properties().iter().enumerate() {
                writer.property(i, prop.name, &prop.value.0)?;
            }
            writer.feature_end(0)?;
        }
        writer.write(self.buf)
    }
}
