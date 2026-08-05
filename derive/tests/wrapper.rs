//! Integration tests for the `algosul-derive::Wrapper` derive macro.
//! Run with `--features wrapper` (see the `[[test]] required-features`
//! entry in Cargo.toml).

use algosul_core::wrapper::prelude::{Inner, InnerMut, Wrapper};
use algosul_derive::Wrapper;

/// A tuple (unnamed-field) struct: the `#[wrapper(inner)]`-marked field
/// (the 0th one, `i64`). The extra `u8` field verifies that the derived
/// macro correctly ignores unnamed fields that are not `inner`.
/// This case previously reproduced a compile defect (a `usize` suffix
/// was appended to the field index, causing
/// `error: suffixes on a tuple index are invalid`), which is now fixed.
#[derive(Debug, Wrapper)]
struct Tuple(#[wrapper(inner)] i64, u8);

/// A named-field struct with the `#[wrapper(inner)]`-marked inner field.
/// The extra `_padding` field only verifies that the derived macro
/// correctly ignores fields that are not `inner`.
#[derive(Debug, Wrapper)]
struct Named
{
  #[wrapper(inner)]
  inner:    u32,
  _padding: u64,
}

/// A struct with generics.
#[derive(Debug, Wrapper)]
struct Generic<T, const N: usize>
{
  #[wrapper(inner)]
  inner: [T; N],
}

#[test]
fn named_struct_inner_access()
{
  let mut x = Named { inner: 42, _padding: 7 };
  assert_eq!(x.inner(), &42);
  *x.inner_mut() = 100;
  assert_eq!(*x.inner(), 100);
}

#[test]
fn named_struct_wrapper_associated_type()
{
  let x = Named { inner: 3, _padding: 0 };
  fn accepts(_: impl Wrapper<Inner = u32>) {}
  accepts(x);
}

#[test]
fn generic_struct_derives_correctly()
{
  let mut x = Generic { inner: [1u8, 2, 3] };
  assert_eq!(x.inner(), &[1u8, 2, 3]);
  x.inner_mut()[1] = 99;
  assert_eq!(x.inner(), &[1u8, 99, 3]);
}

#[test]
fn tuple_struct_inner_access()
{
  let mut t = Tuple(7, 99);
  assert_eq!(t.inner(), &7i64);
  *t.inner_mut() = -5;
  assert_eq!(*t.inner(), -5);
  assert_eq!(t.1, 99); // for warn(dead_code)
}

#[test]
fn tuple_struct_wrapper_associated_type()
{
  let t = Tuple(1, 0);
  fn accepts(_: impl Wrapper<Inner = i64>) {}
  accepts(t);
}
