use proc_macro2::Ident;
use syn::{
  Generics,
  Item,
  Token,
  braced,
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  token::Brace,
};
// #[derive(Clone)]
// struct TemplateParameter
// {
//   dollar_token:   Token![$],
//   ident:          Ident,
//   parameter_type: Option<ParameterType>,
// }
//
// #[derive(Clone)]
// struct ParameterType
// {
//   colon_token: Token![:],
//   real_type:   Type,
// }
//
// #[derive(Clone)]
// struct TemplateArgument
// {
//   dollar_token: Token![$],
//   ident:        Ident,
// }
//
// #[derive(Clone)]
// enum TemplateNameItem
// {
//   Ident(Ident),
//   TemplateArgument(TemplateArgument),
// }
//
// #[derive(Clone)]
// enum TemplateName
// {
//   Ident(Ident),
//   Template
//   {
//     lt_token:          Token![<],
//     template_argument: Punctuated<TemplateName, Token![,]>,
//     gt_token:          Token![>],
//   },
// }

#[derive(Clone)]
pub(crate) struct TemplateInput
{
  enums:       Vec<EnumDef>,
  impl_blocks: Vec<ImplBlock>,
}

#[derive(Clone)]
struct EnumDef
{
  enum_token: Token![enum],
  name:       Ident,
  brace:      Brace,
  variants:   Punctuated<Ident, Token![,]>,
}

#[derive(Clone)]
struct ImplBlock
{
  impl_token: Token![impl],
  generics:   Generics,
  brace:      Brace,
  items:      Punctuated<Item, Token![;]>,
}

impl Parse for TemplateInput
{
  fn parse(input: ParseStream) -> syn::Result<Self>
  {
    let mut enums = Vec::new();
    let mut impl_blocks = Vec::new();
    if input.peek(Token![enum])
    {
      enums.push(input.parse()?);
    }
    else if input.peek(Token![impl])
    {
      impl_blocks.push(input.parse()?);
    }
    Ok(Self { enums, impl_blocks })
  }
}

impl Parse for EnumDef
{
  fn parse(input: ParseStream) -> syn::Result<Self>
  {
    let content;
    Ok(Self {
      enum_token: input.parse()?,
      name:       input.parse()?,
      brace:      braced!(content in input),
      variants:   content.parse_terminated(Ident::parse, Token![,])?,
    })
  }
}

impl Parse for ImplBlock
{
  fn parse(input: ParseStream) -> syn::Result<Self>
  {
    let content;
    Ok(Self {
      impl_token: input.parse()?,
      generics:   input.parse()?,
      brace:      braced!(content in input),
      items:      content.parse_terminated(Item::parse, Token![;])?,
    })
  }
}

// #[derive(Clone)]
// pub(crate) struct Template
// {
//   template_token:     Token![impl],
//   lt_token:           Token![<],
//   template_parameter: Punctuated<TemplateParameter, Token![,]>,
//   gt_token:           Token![>],
//   template_name:      TemplateName,
//   eq_token:           Token![=],
// }
//
// impl Parse for Template
// {
//   fn parse(input: ParseStream) -> syn::Result<Self>
//   {
//     Ok(Self {
//       template_token:     input.parse()?,
//       lt_token:           input.parse()?,
//       template_parameter: input.parse()?,
//       gt_token:           input.parse()?,
//       template_name:      input.parse()?,
//       eq_token:           input.parse()?,
//     })
//   }
// }

// impl Parse for TemplateParameter
// {
//   fn parse(input: ParseStream) -> syn::Result<Self>
//   {
//     Ok(Self {
//       ident:          input.parse()?,
//       dollar_token:   input.parse()?,
//       parameter_type: input.parse()?,
//     })
//   }
// }
//
// impl Parse for ParameterType
// {
//   fn parse(input: ParseStream) -> syn::Result<Self>
//   {
//     Ok(Self { colon_token: input.parse()?, real_type: input.parse()? })
//   }
// }
//
// impl Parse for TemplateName
// {
//   fn parse(input: ParseStream) -> syn::Result<Self>
//   {
//     match input.parse::<Ident>()
//     {
//       Ok(ident) => Ok(Self::Ident(ident)),
//       Err(err) =>
//       {
//         let content;
//         Ok(Self::Template {
//           gt_token:          input.parse()?,
//           template_argument: input.parse_terminated(Type::parse, Token![,])?,
//           lt_token:          input.parse()?,
//         })
//       }
//     }
//   }
// }
