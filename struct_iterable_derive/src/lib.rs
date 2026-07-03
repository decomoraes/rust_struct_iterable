//! Implementation detail of [`struct_iterable`](https://docs.rs/struct_iterable).
//!
//! Depend on `struct_iterable` instead of this crate; its API is only
//! guaranteed through the facade.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path};

/// Derive macro that makes a struct's fields iterable as
/// `(&'static str, &dyn Any)` pairs.
///
/// # Limitations
///
/// - Only structs with named fields are supported (no enums, unions, tuple
///   structs, or unit structs).
/// - Every field type must satisfy `Any`'s `'static` requirement to be
///   type-erased. Structs with type or lifetime parameters compile, and the
///   `Iterable` impl applies whenever each field type is `'static` (e.g.
///   `Wrapper<String>` or `Borrowed<'static>`).
///
/// # Container attributes
///
/// - `#[iterable(crate = "path")]` — path to the crate that exports the
///   `Iterable` trait, when `struct_iterable` is renamed in your `Cargo.toml`
///   or re-exported from another crate. Defaults to `::struct_iterable`.
///
/// # Example
///
/// ```
/// use struct_iterable::Iterable;
///
/// #[derive(Iterable)]
/// struct MyStruct {
///     field1: i32,
///     field2: String,
/// }
///
/// let my_instance = MyStruct {
///     field1: 42,
///     field2: "Hello, world!".to_string(),
/// };
///
/// for (field_name, field_value) in my_instance.iter() {
///     println!("{}: {:?}", field_name, field_value);
/// }
/// ```
#[proc_macro_derive(Iterable, attributes(iterable))]
pub fn derive_iterable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let crate_path = parse_container_attrs(&input.attrs)?
        .unwrap_or_else(|| syn::parse_quote!(::struct_iterable));

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => &named.named,
            Fields::Unnamed(_) | Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "`#[derive(Iterable)]` currently supports only structs with named fields",
                ));
            }
        },
        Data::Enum(data) => {
            return Err(syn::Error::new_spanned(
                data.enum_token,
                "`#[derive(Iterable)]` cannot be derived for enums; \
                 it iterates over the fields of a struct",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new_spanned(
                data.union_token,
                "`#[derive(Iterable)]` cannot be derived for unions",
            ));
        }
    };

    deny_field_attrs(fields)?;

    let struct_name = &input.ident;

    // `dyn Any` requires every erased value to be `'static`, so each field
    // type carries its own `FieldTy: Any` predicate, spanned to that field's
    // type: an unsatisfied bound underlines the offending field at the use
    // site instead of rejecting the whole derive.
    let mut generics = input.generics.clone();
    {
        let where_clause = generics.make_where_clause();
        let mut seen = HashSet::new();
        for field in fields {
            let ty = &field.ty;
            if seen.insert(quote!(#ty).to_string()) {
                where_clause.predicates.push(syn::parse_quote_spanned!(
                    ty.span() => #ty: ::std::any::Any
                ));
            }
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields_iter = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        quote! {
            (#field_name, &(self.#field_ident) as &dyn ::std::any::Any)
        }
    });

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #crate_path::Iterable for #struct_name #ty_generics #where_clause {
            fn iter(
                &self,
            ) -> ::std::vec::IntoIter<(&'static str, &'_ dyn ::std::any::Any)> {
                ::std::vec![
                    #(#fields_iter),*
                ]
                .into_iter()
            }
        }
    })
}

fn parse_container_attrs(attrs: &[syn::Attribute]) -> syn::Result<Option<Path>> {
    let mut crate_path = None;
    for attr in attrs {
        if !attr.path().is_ident("iterable") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                crate_path = Some(lit.parse()?);
                Ok(())
            } else {
                Err(meta.error("unknown attribute; expected `#[iterable(crate = \"...\")]`"))
            }
        })?;
    }
    Ok(crate_path)
}

fn deny_field_attrs(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> syn::Result<()> {
    let mut error: Option<syn::Error> = None;
    for field in fields {
        for attr in &field.attrs {
            if attr.path().is_ident("iterable") {
                let new = syn::Error::new_spanned(
                    attr,
                    "field-level `#[iterable(...)]` attributes are not supported yet",
                );
                match &mut error {
                    Some(error) => error.combine(new),
                    None => error = Some(new),
                }
            }
        }
    }
    match error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
