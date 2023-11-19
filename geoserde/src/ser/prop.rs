pub trait PropertySink {}
pub struct PropertySerializer<T> {
    sink: T,
}
impl<T: PropertySink> PropertySerializer<T> {
    pub fn new(sink: T) -> Self {
        Self { sink }
    }
}
