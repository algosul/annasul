#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::pin::Pin;

/// Prepare for dynamically dispatched async methods.
/// # Examples
/// ```
/// # use std::io::Result;
/// # use algosul_core::util::r#async::PinBoxDynFeature;
///
/// pub trait AsyncRead
/// {
///   fn read(&mut self) -> PinBoxDynFeature<Result<u8>>;
/// }
///
/// impl AsyncRead for ()
/// {
///   fn read(&mut self) -> PinBoxDynFeature<Result<u8>>
///   {
///     Box::pin(async move { Ok(0) })
///   }
/// }
/// let _: &dyn AsyncRead = &();
/// ```
pub type PinBoxDynFeature<T> =
  Pin<Box<dyn Future<Output = T> + Send + 'static>>;
