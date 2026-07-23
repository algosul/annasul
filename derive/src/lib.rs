use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[cfg(feature = "wrapper")]
mod wrapper;

#[cfg(feature = "wrapper")]
#[proc_macro_derive(Wrapper, attributes(wrapper))]
pub fn wrapper_derive(token_stream: TokenStream) -> TokenStream
{
  let input = parse_macro_input!(token_stream as DeriveInput);
  TokenStream::from(wrapper::WrapperParser::new().parse(&input))
}
