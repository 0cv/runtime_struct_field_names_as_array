#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{
  parse_macro_input, DeriveInput, Meta, PathArguments, Type,
};

/// Derives the `FieldNamesAsArray` procedural macro.
///
/// # Panics
///
/// If the token stream is not coming from a named struct or if
/// the `field_names_as_array` attribute is used wrongfully, deriving
/// this macro will fail.
///
#[proc_macro_derive(
  FieldNamesAsArray,
  attributes(field_names_as_array)
)]
pub fn derive_field_names_as_array(
  input: TokenStream,
) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let struct_name = &input.ident;
  let fields = match &input.data {
    syn::Data::Struct(data) => &data.fields,
    _ => panic!("FieldNamesAsArray can only be derived for structs"),
  };

  let mut field_exprs = Vec::new();

  for field in fields.iter() {
    let field_name = field.ident.as_ref().unwrap().to_string();

    if let Some(attr) = field
      .attrs
      .iter()
      .find(|a| a.path().is_ident("field_names_as_array"))
    {
      let field_type = &field.ty;
      let nested_struct = if let Type::Path(type_path) = field_type {
        if type_path.path.segments.last().unwrap().ident == "Option" {
          if let PathArguments::AngleBracketed(arguments) =
            &type_path.path.segments.last().unwrap().arguments
          {
            if let Some(generic_arg) = arguments.args.first() {
              if let syn::GenericArgument::Type(inner_type) =
                generic_arg
              {
                Some(inner_type)
              } else {
                None
              }
            } else {
              None
            }
          } else {
            None
          }
        } else {
          Some(field_type)
        }
      } else {
        None
      };

      let flatten: bool = match &attr.meta {
        Meta::List(meta) => meta
          .to_owned()
          .tokens
          .into_iter()
          .find(|token| token.to_string() == "flatten")
          .is_some(),
        _ => false,
      };

      if flatten {
        if let Some(nested_struct) = nested_struct {
          field_exprs.push(quote! { <#nested_struct>::field_names_as_array().iter().map(|s| format!("{}.{}", #field_name, s)).collect::<Vec<_>>() });
        }
      } else {
        field_exprs.push(quote! { vec![#field_name.to_string()] });
      }
    } else {
      field_exprs.push(quote! { vec![#field_name.to_string()] });
    }
  }

  let output = quote! {
      impl #struct_name {
          pub fn field_names_as_array() -> Vec<String> {
              let mut field_names = Vec::new();
              #( field_names.extend(#field_exprs); )*
              field_names
          }
      }
  };

  output.into()
}
