use Channel::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Channel
{
  Alpha,
  Red,
  Blue,
  Green,
  Gray,
  H,
  S,
  V,
}

pub trait ColorSpace<const N: usize>
{
  const CHANNELS: [Channel; N];
}

pub trait Alpha
{
  const ALPHA_INDEX: usize;
  type NoAlphaType: NoAlpha<AlphaType = Self>;
}

pub trait NoAlpha
{
  type AlphaType: Alpha<NoAlphaType = Self>;
}

macro_rules! define_space {
  ($($name:ident $alpha_name:ident {
    $n:literal $n_1:literal: $($channel:expr),+ $(,)?
  }),* $(,)?) => {
    $(
      #[derive(Debug, Clone, Copy)]
      pub struct $name;
      #[derive(Debug, Clone, Copy)]
      pub struct $alpha_name;
      impl ColorSpace<$n> for $name {
        const CHANNELS: [Channel; $n] = [$($channel),+];
      }
      impl ColorSpace<$n_1> for $alpha_name {
        const CHANNELS: [Channel; $n_1] = [$($channel),+, Alpha];
      }
      impl NoAlpha for $name {
        type AlphaType = $alpha_name;
      }
      impl Alpha for $alpha_name {
        const ALPHA_INDEX: usize = $n;
        type NoAlphaType = $name;
      }
    )*
  };
}

define_space! {
  Rgb Rgba {
    3 4: Red, Green, Blue
  },
  Bgr Bgra {
    3 4: Blue, Red, Green
  },
  Hsv Hsva {
    3 4: H, S, V
  },
  G Ga {
    1 2: Gray
  },
}
