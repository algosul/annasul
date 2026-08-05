use std::simd::SimdElement;

use crate::traits::ops::Cross;
use crate::vector::Vector;
use algosul_core::wrapper::prelude::*;
use algosul_derive::Wrapper;
use traits::IsInside;

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "__feature-portable_simd"))]
compile_error!("no feature '__feature-portable_simd'");

#[cfg(not(feature = "__feature-algosul-wrapper"))]
compile_error!("no feature '__feature-algosul-wrapper'");

#[cfg(not(feature = "__feature-const_trait_impl"))]
compile_error!("no feature '__feature-const_trait_impl'");

pub mod traits;

#[derive(Copy, Clone, Debug, Wrapper)]
pub struct Triangle<T, const N: usize>
where
  T: SimdElement,
{
  #[wrapper(inner)]
  points: [Vector<T, N>; 3],
}

impl<T, const N: usize> Triangle<T, N>
where
  T: SimdElement,
{
  #[inline]
  pub const fn from_array(array: [Vector<T, N>; 3]) -> Self
  {
    Self::from_inner(array)
  }
}

const impl<T, const N: usize> FromInner for Triangle<T, N>
where
  T: SimdElement,
{
  #[inline]
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { points: inner }
  }
}

macro_rules! impl_shape_for_triangle_int {
  ($ty:ty) => {
    impl IsInside<$ty, 2> for Triangle<$ty, 2>
    {
      #[inline]
      fn is_inside(&self, point: Vector<$ty, 2>) -> bool {
        let s1 = (self.points[1] - self.points[0]).cross(point - self.points[0]).signum();
        let s2 = (self.points[2] - self.points[1]).cross(point - self.points[1]).signum();

        if s1 != 0 && s2 != 0 && s1 != s2 {
          return false;
        }

        let s3 = (self.points[0] - self.points[2]).cross(point - self.points[2]).signum();
        (s1 != 1 && s2 != 1 && s3 == 1) || (s1 != -1 && s2 != -1 && s3 != -1)
      }
    }
    impl IsInside<$ty, 3> for Triangle<$ty, 3>
    {
      #[inline]
      fn is_inside(&self, _point: Vector<$ty, 3>) -> bool {
        todo!()
      }
    }
    impl IsInside<$ty, 4> for Triangle<$ty, 4>
    {
      #[inline]
      fn is_inside(&self, _point: Vector<$ty, 4>) -> bool {
        todo!()
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_shape_for_triangle_int!($ty);)+
  };
}

macro_rules! impl_shape_for_triangle_float {
  ($ty:ty) => {
    impl IsInside<$ty, 2> for Triangle<$ty, 2>
    {
      #[inline]
      fn is_inside(&self, point: Vector<$ty, 2>) -> bool {
        let s1 = (self.points[1] - self.points[0]).cross(point - self.points[0]).signum();
        let s2 = (self.points[2] - self.points[1]).cross(point - self.points[1]).signum();

        if s1 != 0.0 && s2 != 0.0 && s1 != s2 {
          return false;
        }

        let s3 = (self.points[0] - self.points[2]).cross(point - self.points[2]).signum();
        (s1 != 1.0 && s2 != 1.0 && s3 == 1.0) || (s1 != -1.0 && s2 != -1.0 && s3 != -1.0)
      }
    }
    impl IsInside<$ty, 3> for Triangle<$ty, 3>
    {
      #[inline]
      fn is_inside(&self, _point: Vector<$ty, 3>) -> bool {
        todo!()
      }
    }
    impl IsInside<$ty, 4> for Triangle<$ty, 4>
    {
      #[inline]
      fn is_inside(&self, _point: Vector<$ty, 4>) -> bool {
        todo!()
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_shape_for_triangle_float!($ty);)+
  };
}

impl_shape_for_triangle_int! {
  i8
  i16
  i32
  i64
}

impl_shape_for_triangle_float! {
  f32
  f64
}

#[cfg(test)]
mod tests
{
  use super::traits::IsInside;
  use super::*;
  use crate::vector::Vector;
  use algosul_core::wrapper::Inner;

  fn triangle_i32() -> Triangle<i32, 2>
  {
    Triangle::from_array([
      Vector::<i32, 2>::from_array([0, 0]),
      Vector::<i32, 2>::from_array([4, 0]),
      Vector::<i32, 2>::from_array([0, 4]),
    ])
  }

  fn triangle_f32() -> Triangle<f32, 2>
  {
    Triangle::from_array([
      Vector::<f32, 2>::from_array([0.0, 0.0]),
      Vector::<f32, 2>::from_array([4.0, 0.0]),
      Vector::<f32, 2>::from_array([0.0, 4.0]),
    ])
  }

  #[test]
  fn from_array_stores_points()
  {
    let t = triangle_i32();
    let pts = t.inner(); // &[Vector<i32,2>; 3]
    assert_eq!(pts[0].inner().to_array(), [0, 0]);
    assert_eq!(pts[1].inner().to_array(), [4, 0]);
    assert_eq!(pts[2].inner().to_array(), [0, 4]);
  }

  #[test]
  fn point_inside_int_triangle()
  {
    let t = triangle_i32();
    // Interior points
    assert!(t.is_inside(Vector::<i32, 2>::from_array([1, 1])));
    assert!(t.is_inside(Vector::<i32, 2>::from_array([3, 1])));
    assert!(t.is_inside(Vector::<i32, 2>::from_array([1, 3])));
    // Clearly outside
    assert!(!t.is_inside(Vector::<i32, 2>::from_array([100, 100])));
  }

  #[test]
  fn point_inside_float_triangle()
  {
    let t = triangle_f32();
    assert!(t.is_inside(Vector::<f32, 2>::from_array([1.0, 1.0])));
    assert!(t.is_inside(Vector::<f32, 2>::from_array([0.5, 0.5])));
    assert!(!t.is_inside(Vector::<f32, 2>::from_array([-1.0, -1.0])));
    assert!(!t.is_inside(Vector::<f32, 2>::from_array([5.0, 5.0])));
  }
}
