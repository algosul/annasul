#[cfg(feature = "serialization-rykv")]
pub(crate) mod rkyv;
#[cfg(feature = "serialization-serde")]
pub(crate) mod serde;

#[cfg(not(any(
  feature = "serialization-rykv",
  feature = "serialization-serde"
)))]
compile_error!("not set feature `serialization-xyz`");
