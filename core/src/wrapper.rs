pub trait Wrapper<I: ?Sized>
{
  fn inner(&self) -> &I;
  fn inner_mut(&mut self) -> &mut I;
}
pub trait WrapperOwned<I>: Wrapper<I>
{
  fn into_inner(self) -> I;
}
