#[macro_export]
macro_rules! type_defines {
  ($ty:ident { $($(#[$meta:meta])? $name:ident($($t:ty),* $(, $($i:literal),*)? $(,)?)),+ $(,)? } ) => {
    $(
      $(#[$meta])?
      pub type $name = $ty<$($t),* $(, $($i),*)?>;
    )+
  };
}

#[macro_export]
macro_rules! impl_element_getter {
  ($ty:ident { $($name:ident: $i:literal $($n:literal)+),+ $(,)? } ) => {
    $(
      $(impl<T: SimdElement> $ty<T, $n>
      {
        pub fn $name(&self) -> T
        {
          self.inner[$i]
        }
      })+
    )+
  };
}

#[macro_export]
macro_rules! impl_channel_getter {
  ($ty:ident { $($name:ident: $i:literal $($n:literal)+),+ $(,)? } ) => {
    $(
      $(impl<T, S> $ty<T, S, $n>
        where
          T: SimdElement,
          S: ColorSpace<$n>,
      {
        pub fn $name(&self) -> T
        {
          self.inner[$i]
        }
      })+
    )+
  };
}