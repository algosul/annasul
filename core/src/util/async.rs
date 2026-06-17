use std::pin::Pin;

pub type PinBoxDynFeature<T> =
  Pin<Box<dyn Future<Output = T> + Send + 'static>>;
