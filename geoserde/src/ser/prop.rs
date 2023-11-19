pub trait PropertySink {}
impl<T: geozero::PropertyProcessor> PropertySink for T {}
pub struct PropertySerializer<'a, T> {
    sink: &'a T,
}
impl<'a, T: PropertySink> PropertySerializer<'a, T> {
    pub fn new(sink: &'a T) -> Self {
        Self { sink }
    }
}
