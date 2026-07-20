use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_warning::FormattedWarning;
use quote::{ToTokens, quote};
use syn::{
  Data,
  DataStruct,
  DeriveInput,
  Error,
  Field,
  Type,
  Visibility,
  spanned::Spanned,
};

#[derive(Debug, Default, Clone)]
pub struct WrapperParser
{
  warnings: Vec<FormattedWarning>,
  errors:   Vec<Error>,
}

impl WrapperParser
{
  pub fn new() -> Self
  {
    Self { ..Default::default() }
  }

  pub(crate) fn parse(&mut self, derive_input: &DeriveInput) -> TokenStream
  {
    let impl_token_stream = self.for_derive_input(derive_input);

    let errors: Vec<_> =
      self.errors.iter().map(Error::to_compile_error).collect();
    let warnings = &self.warnings;

    quote! {
      #(#errors)*
      #(#warnings)*
      #impl_token_stream
    }
  }

  fn for_derive_input(&mut self, derive_input: &DeriveInput) -> TokenStream
  {
    let name = &derive_input.ident;
    match &derive_input.data
    {
      Data::Struct(data) =>
      {
        let inner_type = self.for_struct(data, derive_input.span());

        quote! {
          impl ::algosul_core::wrapper::Wrapper for #name {
            type Inner = #inner_type;
          }
        }
      }
      Data::Enum(_data) =>
      {
        todo!()
      }
      _ => Error::new_spanned(derive_input, "`Wrapper` can't use for union")
        .into_compile_error(),
    }
  }

  fn attr_inner_check(&mut self, arg: &Ident, i: usize, field: &Field)
  {
    if let Visibility::Public(_) = field.vis
    {
      let ident_name =
        field.ident.as_ref().map(Ident::to_string).unwrap_or(i.to_string());
      self
        .errors
        .push(Error::new_spanned(arg, format!("public field `{ident_name}`")));
      self.errors.push(Error::new_spanned(
        &field.vis,
        format!("public field `{ident_name}`"),
      ));
    }
  }

  fn for_struct_field(
    &mut self, option_field: &mut Option<Field>, i: usize, field: &Field,
  )
  {
    for attr in &field.attrs
    {
      if attr.meta.path().is_ident("wrapper")
      {
        let arg: Ident = if let Ok(arg) = attr.parse_args()
        {
          arg
        }
        else
        {
          self.errors.push(Error::new_spanned(
            attr,
            format!("Invalid attribute: `{}`", attr.to_token_stream()),
          ));
          continue;
        };
        if arg == "inner"
        {
          self.attr_inner_check(&arg, i, field);
          if option_field.is_some()
          {
            self.errors.push(Error::new_spanned(
              attr,
              "Duplicate `#[wrapper]` attribute",
            ));
          }
          else
          {
            *option_field = Some(field.clone());
          }
        }
        else
        {
          self.errors.push(Error::new_spanned(
            &arg,
            format!("Invalid attribute `#[wrapper({arg})]`"),
          ));
        }
      }
    }
  }

  fn for_struct(&mut self, data_struct: &DataStruct, span: Span)
  -> Option<Type>
  {
    let mut option_field = None;
    for (i, field) in data_struct.fields.iter().enumerate()
    {
      self.for_struct_field(&mut option_field, i, field);
    }
    if let Some(field) = option_field
    {
      Some(field.ty)
    }
    else
    {
      self.errors.push(Error::new(span, "no set `#[wrapper(inner)]`"));
      None
    }
  }
}
