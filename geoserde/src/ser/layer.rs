// use super::feat::FeatureSink;

// pub struct LayerSerializer<'a, S: FeatureSink> {
//     sink: &'a mut S,
// }
// impl<'a, S: FeatureSink> LayerSerializer<'a, S> {
//     pub fn new(sink: &'a mut S) -> Self {
//         Self { sink }
//     }
//     pub fn write_features<I, S>(self, features: I)
//     where
//         I: IntoIterator<Item = S>,
//         S: Serialize,
//     {
//     }
// }
