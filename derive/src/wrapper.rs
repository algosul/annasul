use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
  Attribute,
  Data,
  DataStruct,
  DeriveInput,
  Error,
  Field,
  Type,
  Visibility,
  spanned::Spanned,
};

use crate::Message;

#[derive(Default, Clone)]
pub struct WrapperParser
{
  messages: Vec<Message>,
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

    let messages = self.messages.iter().map(Message::to_token_stream);

    quote! {
      #(#messages)*
      #impl_token_stream
    }
  }

  fn for_derive_input(&mut self, derive_input: &DeriveInput) -> TokenStream
  {
    let name = &derive_input.ident;
    let (impl_generics, type_generics, where_clause) =
      derive_input.generics.split_for_impl();
    match &derive_input.data
    {
      Data::Struct(data) =>
      {
        if let Some((inner_index, inner_ident, inner_type)) =
          self.for_struct(data, derive_input.span())
        {
          let inner =
            inner_ident.map(ToTokens::into_token_stream).unwrap_or_else(|| {
              Literal::usize_unsuffixed(inner_index).into_token_stream()
            });
          quote! {
            impl #impl_generics Wrapper for #name #type_generics
              #where_clause
            {
              type Inner = #inner_type;
            }

            impl #impl_generics Inner for #name #type_generics
            #where_clause
            {
              fn inner(&self) -> &Self::Inner
              {
                &self.#inner
              }
            }

            impl #impl_generics InnerMut for #name #type_generics
            #where_clause
            {
              fn inner_mut(&mut self) -> &mut Self::Inner
              {
                &mut self.#inner
              }
            }

          }
        }
        else
        {
          TokenStream::new()
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

  fn attr_inner_check(&mut self, _arg: &Ident, i: usize, field: &Field)
  {
    if let Visibility::Public(_) = field.vis
    {
      self.messages.push(Message::Error(
        crate::Error::WrapperInnerUseForPublicField { field: field.clone() },
      ));
    }
  }

  fn for_struct_field(
    &mut self, option_field: &mut Option<(usize, Option<Ident>, Type)>,
    i: usize, field: &Field,
  )
  {
    field.attrs.iter().for_each(|attr| {
      self.for_struct_field_attr(attr, option_field, i, field);
    });
  }

  fn for_struct_field_attr(
    &mut self, attr: &Attribute,
    option_field: &mut Option<(usize, Option<Ident>, Type)>, i: usize,
    field: &Field,
  )
  {
    if !attr.meta.path().is_ident("wrapper")
    {
      return;
    }
    let arg: Ident = if let Ok(arg) = attr.parse_args()
    {
      arg
    }
    else
    {
      self.messages.push(Message::Error(crate::Error::InvalidAttribute {
        attr: attr.clone(),
      }));
      return;
    };
    if arg == "inner"
    {
      self.attr_inner_check(&arg, i, field);
      if option_field.is_some()
      {
        self.messages.push(Message::Error(
          crate::Error::DuplicateWrapperInnerAttribute { attr: attr.clone() },
        ));
      }
      else
      {
        *option_field = Some((i, field.ident.clone(), field.ty.clone()));
      }
    }
    else
    {
      self.messages.push(Message::Error(crate::Error::InvalidAttribute {
        attr: attr.clone(),
      }));
    }
  }

  fn for_struct(
    &mut self, data_struct: &DataStruct, span: Span,
  ) -> Option<(usize, Option<Ident>, Type)>
  {
    let mut option_field = None;
    for (i, field) in data_struct.fields.iter().enumerate()
    {
      self.for_struct_field(&mut option_field, i, field);
    }

    option_field.or_else(|| {
      self.messages.push(Message::Error(crate::Error::NoWrapperInner { span }));
      None
    })
  }
}
