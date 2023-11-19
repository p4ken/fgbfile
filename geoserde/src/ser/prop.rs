pub trait PropertySink {
    type Error: std::error::Error;
}
#[cfg(feature = "geozero")]
impl<G: geozero::GeomProcessor> PropertySink for G {
    type Error = geozero::error::GeozeroError;
}
pub struct PropertySerializer<'a, T> {
    sink: &'a mut T,
}
impl<'a, T: PropertySink> PropertySerializer<'a, T> {
    pub fn new(sink: &'a mut T) -> Self {
        Self { sink }
    }
}
