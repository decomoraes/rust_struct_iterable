extern crate proc_macro;

use proc_macro::TokenStream; // Substitua esta linha
                             // use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Ident}; // , Type};

#[proc_macro_derive(Iterable)]
pub fn make_iterable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let fields_iter = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_name = field_ident.as_ref().unwrap().to_string();
        quote! {
            (#field_name, &(self.#field_ident) as &dyn std::any::Any)
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn iter_fields<'a>(&'a self) -> std::vec::IntoIter<(&'static str, &'a dyn std::any::Any)> {
                vec![
                    #(#fields_iter),*
                ].into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_make_iterable_macro() {
//         let input: proc_macro2::TokenStream = quote! {
//             pub struct TestStruct {
//                 pub field1: Option<String>,
//                 pub field2: Option<u32>,
//             }
//         };

//         let expected_output = quote! {
//             impl TestStruct {
//                 pub fn iter_fields(&self) -> std::vec::IntoIter<(&str, &dyn std::any::Any)> {
//                     vec![
//                         ("field1", self.field1.as_ref() as &dyn std::any::Any),
//                         ("field2", self.field2.as_ref() as &dyn std::any::Any),
//                     ].into_iter()
//                 }
//             }
//         };

//         let output = make_iterable(input.clone().into());
//         let output = TokenStream2::from(output); // Convert proc_macro::TokenStream to proc_macro2::TokenStream
//         let output = quote! { #output };
//         assert_eq!(expected_output.to_string(), output.to_string());
//     }
// }
