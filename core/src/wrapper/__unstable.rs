use crate::wrapper::Wrapper;

pub const trait FromInner: Wrapper
{
  fn from_inner(inner: Self::Inner) -> Self;
}
