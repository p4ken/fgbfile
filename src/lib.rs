use std::borrow::Cow;
use std::io::Write;
use std::{fs::File, io::BufWriter, path::Path};

use flatgeobuf::geozero::{error::GeozeroError, FeatureProcessor};
use flatgeobuf::{FeatureProperties, FgbWriter, FgbWriterOptions, GeometryType, GeozeroGeometry};

pub struct FgbFile<'a, B> {
    buf: B,
    name: Cow<'a, str>,
    options: FgbWriterOptions<'static>,
}

impl FgbFile<'_, BufWriter<File>> {
    pub fn create(path: &impl AsRef<Path>) -> std::io::Result<FgbFile<BufWriter<File>>> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = BufWriter::new(File::create(path)?);

        let name = path
            .as_ref()
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let options = FgbWriterOptions {
            promote_to_multi: false,
            ..Default::default()
        };

        Ok(FgbFile {
            buf: file,
            name,
            options,
        })
    }
}

impl<W: Write> FgbFile<'_, W> {
    pub fn epsg(mut self, epsg: i32) -> Self {
        self.options.crs.code = epsg;
        self
    }

    pub fn export<T: GeozeroGeometry + FeatureProperties>(
        self,
        features: &[T],
    ) -> Result<(), GeozeroError> {
        let mut writer =
            FgbWriter::create_with_options(&self.name, GeometryType::Unknown, self.options)?;
        for feat in features {
            feat.process_geom(&mut writer)?;
            feat.process_properties(&mut writer)?;
            writer.feature_end(0)?;
        }
        writer.write(self.buf)
    }
}
