use std::simd::num::{SimdFloat, SimdInt, SimdUint};
use std::simd::{Simd, SimdElement, StdFloat};

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

pub trait SimdReduceSum {
  type Scalar;
  fn reduce_sum(self) -> Self::Scalar;
}

pub trait SimdMulAdd<A = Self, B = Self> {
  type Output;
  fn mul_add(self, a: A, b: B) -> Self::Output;
}

pub trait SimdMulAddAssign<A = Self, B = Self> {
  fn mul_add_assign(&mut self, a: A, b: B);
}

macro_rules! impl_simd_traits {
  ($(($ty:ty, $t:ty))+) => {
    $(
      impl<const N: usize> SimdReduceSum for Simd<$ty, N> {
        type Scalar = $ty;
        #[inline]
        fn reduce_sum(self) -> $ty {
          <Self as $t>::reduce_sum(self)
        }
      }
    )+
  };
}

impl_simd_traits! {
  (i8, SimdInt)
  (i16, SimdInt)
  (i32, SimdInt)
  (i64, SimdInt)
  (u8, SimdUint)
  (u16, SimdUint)
  (u32, SimdUint)
  (u64, SimdUint)
  (f32, SimdFloat)
  (f64, SimdFloat)
}
impl<T, const N: usize> SimdMulAdd for Simd<T, N>
where
  T: SimdElement,
  Simd<T, N>: StdFloat,
{
  type Output = Self;
  #[inline]
  fn mul_add(self, a: Self, b: Self) -> Self::Output {
    <Self as StdFloat>::mul_add(self, a, b)
  }
}
impl<T, const N: usize> SimdMulAddAssign for Simd<T, N>
where
  T: SimdElement,
  Simd<T, N>: StdFloat,
{
  #[inline]
  fn mul_add_assign(&mut self, a: Self, b: Self) {
    *self = SimdMulAdd::mul_add(*self, a, b);
  }
}

#[cfg(test)]
mod tests
{
  use std::simd::Simd;

  use super::{SimdMulAdd, SimdMulAddAssign, SimdReduceSum};

  #[test]
  fn reduce_sum_integral()
  {
    let s = Simd::<i32, 4>::from_array([1, 2, 3, 4]);
    assert_eq!(SimdReduceSum::reduce_sum(s), 10);
    let s = Simd::<u8, 4>::from_array([1, 1, 1, 1]);
    assert_eq!(SimdReduceSum::reduce_sum(s), 4);
  }

  #[test]
  fn reduce_sum_float()
  {
    let s = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(SimdReduceSum::reduce_sum(s), 10.0);
  }

  #[test]
  fn mul_add_and_assign()
  {
    let s = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    let a = Simd::<f32, 4>::from_array([2.0, 2.0, 2.0, 2.0]);
    let b = Simd::<f32, 4>::from_array([1.0, 1.0, 1.0, 1.0]);
    let r = SimdMulAdd::mul_add(s, a, b); // s*a + b
    assert_eq!(r.to_array(), [3.0, 5.0, 7.0, 9.0]);
    let mut m = s;
    m.mul_add_assign(a, b);
    assert_eq!(m.to_array(), [3.0, 5.0, 7.0, 9.0]);
  }
}