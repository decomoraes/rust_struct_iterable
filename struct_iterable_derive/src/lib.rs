extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Meta, parse_macro_input, Data, DeriveInput, Fields};

/// The `Iterable` proc macro.
///
/// Deriving this macro for your struct will make it "iterable". An iterable struct allows you to iterate over its fields, returning a tuple containing the field name as a static string and a reference to the field's value as `dyn Any`.
///
/// # Limitations
///
/// - Only structs are supported, not enums or unions.
/// - Only structs with named fields are supported.
///
/// # Usage
///
/// Add the derive attribute (`#[derive(Iterable)]`) above your struct definition.
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
/// ```
///
/// You can now call the `iter` method on instances of your struct to get an iterator over its fields:
///
/// ```
/// let my_instance = MyStruct {
///     field1: 42,
///     field2: "Hello, world!".to_string(),
/// };
///
/// for (field_name, field_value) in my_instance.iter() {
///     println!("{field_name}: {field_value:?}");
/// }
/// ```
#[proc_macro_derive(Iterable, attributes(iterable))]
pub fn derive_iterable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let trait_name = match input.attrs.iter().find(
        |a| a.path().segments.len() == 1 && a.path().segments[0].ident == "iterable"
    ) {
        Some(attr) => {
            if let Meta::List(meta) = &attr.meta {
                meta.tokens.clone()
            } else {
                panic!("Invalid format of \"iterable\" attribute");
            }
        },
        None => {
            quote!(std::any::Any)
        },
    };

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
            (#field_name, &(self.#field_ident) as &dyn #trait_name)
        }
    });

    TokenStream::from(quote! {
        impl<'a> struct_iterable::Iterable for &'a #struct_name {
            type Item = &'a dyn #trait_name;

            fn iter(self) -> std::vec::IntoIter<(&'static str, Self::Item)> {
                vec![
                    #(#fields_iter),*
                ].into_iter()
            }
        }
    })
}
