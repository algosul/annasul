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
  ($ty:ident { $($(#[$meta:meta])? $name:ident: $i:literal),+ $(,)? } ) => {
    $(
      impl<T: SimdElement> $ty<T, $i>
      {
        $(#[$meta])?
        pub fn $name(&self) -> T
        {
          self.inner[$i - 1]
        }
      }
    )+
  };
}
