pub const trait NormSquared
{
  type Output;
  /// Square of the vector norm (length)
  fn norm_squared(self) -> Self::Output;
  /// Square of the vector norm (length)
  /// see [Self::norm_squared]
  fn length_squared(self) -> Self::Output
  where Self: Sized
  {
    self.norm_squared()
  }
}

pub const trait Norm
{
  type Output;
  /// vector norm (length)
  fn norm(self) -> Self::Output;
  /// vector norm (length)
  /// see [Self::norm]
  fn length(self) -> Self::Output
  where Self: Sized
  {
    self.norm()
  }
}

pub const trait INorm
{
  type Output;
  /// vector norm (length)
  fn inorm(self) -> Self::Output;
  /// vector norm (length)
  /// see [Self::inorm]
  fn ilength(self) -> Self::Output
  where Self: Sized
  {
    self.inorm()
  }
}

pub const trait CheckedINorm
{
  type Output;
  /// vector norm (length)
  fn checked_inorm(self) -> Option<Self::Output>;
  /// vector norm (length)
  /// see [Self::checked_inorm]
  fn checked_ilength(self) -> Option<Self::Output>
  where Self: Sized
  {
    self.checked_inorm()
  }
}
