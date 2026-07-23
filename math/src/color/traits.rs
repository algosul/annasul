use std::simd::SimdElement;

use super::{
  Color,
  space::{self, ColorSpace},
};

pub trait Gray<T>
{
  fn gray(&self) -> T;
}
pub trait Rgb<T>
{
  fn r(&self) -> T;
  fn g(&self) -> T;
  fn b(&self) -> T;
}
pub trait Hsv<T>
{
  fn h(&self) -> T;
  fn s(&self) -> T;
  fn v(&self) -> T;
}
pub trait Alpha<T>
{
  fn a(&self) -> T;
}

impl<T, S, const N: usize> Alpha<T> for Color<T, S, N>
where
  T: SimdElement,
  S: ColorSpace<N> + space::Alpha,
{
  fn a(&self) -> T
  {
    self.inner[S::ALPHA_INDEX]
  }
}

impl<T> Rgb<T> for Color<T, space::Rgb, 3>
where T: SimdElement
{
  fn r(&self) -> T
  {
    self.inner[0]
  }

  fn g(&self) -> T
  {
    self.inner[1]
  }

  fn b(&self) -> T
  {
    self.inner[2]
  }
}

impl<T> Rgb<T> for Color<T, space::Rgba, 4>
where T: SimdElement
{
  fn r(&self) -> T
  {
    self.inner[0]
  }

  fn g(&self) -> T
  {
    self.inner[1]
  }

  fn b(&self) -> T
  {
    self.inner[2]
  }
}
impl<T> Hsv<T> for Color<T, space::Hsv, 3>
where T: SimdElement
{
  fn h(&self) -> T
  {
    self.inner[0]
  }

  fn s(&self) -> T
  {
    self.inner[1]
  }

  fn v(&self) -> T
  {
    self.inner[2]
  }
}

impl<T> Hsv<T> for Color<T, space::Hsva, 4>
where T: SimdElement
{
  fn h(&self) -> T
  {
    self.inner[0]
  }

  fn s(&self) -> T
  {
    self.inner[1]
  }

  fn v(&self) -> T
  {
    self.inner[2]
  }
}

impl<T> Gray<T> for Color<T, space::G, 1>
where T: SimdElement
{
  fn gray(&self) -> T
  {
    self.inner[0]
  }
}

impl<T> Gray<T> for Color<T, space::Ga, 2>
where T: SimdElement
{
  fn gray(&self) -> T
  {
    self.inner[0]
  }
}
