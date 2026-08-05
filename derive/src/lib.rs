use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse_macro_input};

#[cfg(feature = "wrapper")]
mod wrapper;

mod template;

#[cfg(feature = "wrapper")]
#[proc_macro_derive(Wrapper, attributes(wrapper))]
pub fn wrapper_derive(token_stream: TokenStream) -> TokenStream
{
  use syn::{DeriveInput, parse_macro_input};
  let input = parse_macro_input!(token_stream as DeriveInput);
  TokenStream::from(wrapper::WrapperParser::new().parse(&input))
}

#[proc_macro]
pub fn template(token_stream: TokenStream) -> TokenStream
{
  TokenStream::from(template::TemplateParser::new().parse(token_stream.into()))
}

#[proc_macro_attribute]
pub fn template_fn(attr: TokenStream, item: TokenStream) -> TokenStream
{
  let parser = syn::meta::parser(|meta| {
    eprintln!("meta: {:#?}", meta.input);
    Ok(())
  });
  parse_macro_input!(attr with parser);
  let item_fn = parse_macro_input!(item as ItemFn);
  eprintln!("item_fn: {:#?}", item_fn.to_token_stream());
  item_fn.into_token_stream().into()
}
