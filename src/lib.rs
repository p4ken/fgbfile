use std::{
    borrow::Cow,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use err::FgbFileError;
use flatgeobuf::{FgbWriter, FgbWriterOptions, GeometryType};
use geoserde::ser::FeatureSerializer;
use serde::Serialize;

mod err;

pub struct FgbFile<'a, B> {
    buf: B,
    name: Cow<'a, str>,
    options: FgbWriterOptions<'static>,
}

impl<'a> FgbFile<'a, BufWriter<File>> {
    /// Create a new FlatGeobuf file in the local path.
    pub fn create(path: &'a (impl AsRef<Path> + ?Sized)) -> std::io::Result<Self> {
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
    /// If you want to write to a file, use [create] instead.
    pub fn new(buf: W, name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            buf,
            name: name.into(),
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

    /// Write serializable features.
    pub fn write_features<I, S>(self, features: I) -> Result<usize, FgbFileError>
    where
        I: IntoIterator<Item = S>,
        S: Serialize,
    {
        let mut writer =
            FgbWriter::create_with_options(&self.name, GeometryType::Unknown, self.options)?;
        let count;
        {
            let mut ser = FeatureSerializer::new(&mut writer);
            for feat in features {
                feat.serialize(&mut ser)?;
            }
            count = ser.count();
        }
        writer.write(self.buf)?;
        Ok(count)
    }
}
