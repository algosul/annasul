#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::pin::Pin;

pub type PinBoxDynFeature<T> =
  Pin<Box<dyn Future<Output = T> + Send + 'static>>;
