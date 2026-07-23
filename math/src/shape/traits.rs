use std::simd::SimdElement;

use crate::vector::Vector;

pub trait IsInside<T, const N: usize>
where T: SimdElement
{
  fn is_inside(&self, point: Vector<T, N>) -> bool;
}
