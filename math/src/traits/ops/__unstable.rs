pub const trait Dot<Rhs = Self>
{
  type Output;
  /// vector dot product
  fn dot(self, rhs: Rhs) -> Self::Output;
}

pub const trait Cross<Rhs = Self>
{
  type Output;
  /// vector cross product
  /// `(x, y, z) = (x1, y1, z1) x (x2, y2, z2)`
  fn cross(self, rhs: Rhs) -> Self::Output;
}
