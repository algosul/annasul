use algosul_derive_util::Logger;
use proc_macro2::TokenStream;
use quote::quote;

mod ast;

#[derive(Default, Debug, Clone)]
pub(crate) struct TemplateParser
{
  logger: Logger,
}

impl TemplateParser
{
  pub(crate) fn new() -> Self
  {
    Self { ..Default::default() }
  }

  pub(crate) fn parse(&mut self, token_stream: TokenStream) -> TokenStream
  {
    match syn::parse2::<ast::TemplateInput>(token_stream)
    {
      Ok(template) => todo!(),
      Err(err) =>
      {
        self.logger.syn_error(err);
      }
    }
    let logger = &self.logger;
    quote! {
      #logger
    }
  }
}
