use proc_macro2::TokenStream;

mod ast;

#[derive(Default, Debug, Clone)]
pub(crate) struct TemplateParser {}

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
        todo!()
      }
    }
  }
}
