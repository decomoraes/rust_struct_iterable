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
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path, TypeTraitObject};

/// Derive macro that implements `Iterable` and `IterableMut` for a struct,
/// plus `IterableAs<dyn Trait>` for every `#[iterable(to(dyn Trait))]`.
///
/// See the [`struct_iterable`](https://docs.rs/struct_iterable) documentation
/// for the full attribute reference and examples.
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

struct ContainerAttrs {
    crate_path: Option<Path>,
    to: Vec<TypeTraitObject>,
    no_any: bool,
}

struct FieldInfo<'a> {
    name: String,
    member: syn::Member,
    ty: &'a syn::Type,
}

fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attrs = parse_container_attrs(&input.attrs)?;
    let krate = attrs
        .crate_path
        .clone()
        .unwrap_or_else(|| syn::parse_quote!(::struct_iterable));

    if attrs.no_any && attrs.to.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`#[iterable(no_any)]` without any `to(dyn Trait)` would generate no impls at all",
        ));
    }

    let fields = collect_fields(input)?;

    let mut output = TokenStream2::new();
    if !attrs.no_any {
        output.extend(expand_any_impls(input, &krate, &fields));
    }
    for trait_object in &attrs.to {
        output.extend(expand_as_impl(input, &krate, &fields, trait_object));
    }
    Ok(output)
}

/// Validate the struct shape and flatten its non-skipped fields into
/// (name, member, type) triples, in declaration order.
fn collect_fields(input: &DeriveInput) -> syn::Result<Vec<FieldInfo<'_>>> {
    let data = match &input.data {
        Data::Struct(data) => data,
        Data::Enum(data) => {
            return Err(syn::Error::new_spanned(
                data.enum_token,
                "`#[derive(Iterable)]` cannot be derived for enums; it iterates over the fields \
                 of a struct (for enum variants, see the `strum` crate)",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new_spanned(
                data.union_token,
                "`#[derive(Iterable)]` cannot be derived for unions",
            ));
        }
    };

    let declared: Vec<&syn::Field> = match &data.fields {
        Fields::Named(named) => named.named.iter().collect(),
        Fields::Unnamed(unnamed) => unnamed.unnamed.iter().collect(),
        Fields::Unit => Vec::new(),
    };

    let mut fields = Vec::new();
    let mut seen_names = HashSet::new();
    for (position, field) in declared.into_iter().enumerate() {
        let field_attrs = parse_field_attrs(field)?;
        if field_attrs.skip {
            continue;
        }
        let name = match field_attrs.rename {
            Some(rename) => rename,
            // Raw identifiers drop their `r#` prefix, matching serde: the
            // yielded name is "type", not "r#type".
            None => match &field.ident {
                Some(ident) => {
                    let s = ident.to_string();
                    s.strip_prefix("r#").map(str::to_owned).unwrap_or(s)
                }
                // Tuple fields are named by position, mirroring `self.0`.
                None => position.to_string(),
            },
        };
        if !seen_names.insert(name.clone()) {
            return Err(syn::Error::new_spanned(
                field,
                format!("duplicate field name `{name}`; by-name lookup would be ambiguous"),
            ));
        }
        let member = match &field.ident {
            Some(ident) => syn::Member::Named(ident.clone()),
            None => syn::Member::Unnamed(syn::Index::from(position)),
        };
        fields.push(FieldInfo {
            name,
            member,
            ty: &field.ty,
        });
    }
    Ok(fields)
}

/// The `Iterable` + `IterableMut` impls (the `dyn Any` path).
///
/// `dyn Any` requires every erased value to be `'static`, so each field type
/// carries its own `FieldTy: Any` predicate, spanned to that field's type: an
/// unsatisfied bound underlines the offending field at the use site instead
/// of rejecting the whole derive.
fn expand_any_impls(input: &DeriveInput, krate: &Path, fields: &[FieldInfo]) -> TokenStream2 {
    let struct_name = &input.ident;
    let field_count = fields.len();

    let mut generics = input.generics.clone();
    {
        let where_clause = generics.make_where_clause();
        let mut seen = HashSet::new();
        for field in fields {
            let ty = field.ty;
            if seen.insert(quote!(#ty).to_string()) {
                where_clause.predicates.push(syn::parse_quote_spanned!(
                    ty.span() => #ty: ::core::any::Any
                ));
            }
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let index_param = if field_count == 0 {
        quote!(_index)
    } else {
        quote!(index)
    };

    let arms = fields.iter().enumerate().map(|(i, field)| {
        let name = &field.name;
        let member = &field.member;
        quote! {
            #i => ::core::option::Option::Some(
                (#name, &self.#member as &dyn ::core::any::Any),
            ),
        }
    });
    let arms_mut = fields.iter().enumerate().map(|(i, field)| {
        let name = &field.name;
        let member = &field.member;
        quote! {
            #i => ::core::option::Option::Some(
                (#name, &mut self.#member as &mut dyn ::core::any::Any),
            ),
        }
    });

    quote! {
        #[automatically_derived]
        impl #impl_generics #krate::Iterable for #struct_name #ty_generics #where_clause {
            #[inline]
            fn field_count(&self) -> usize {
                #field_count
            }

            fn field_at(
                &self,
                #index_param: usize,
            ) -> ::core::option::Option<(&'static str, &dyn ::core::any::Any)> {
                match #index_param {
                    #(#arms)*
                    _ => ::core::option::Option::None,
                }
            }

            #[inline]
            fn iter(&self) -> #krate::FieldIter<'_> {
                #krate::FieldIter::new(self)
            }
        }

        #[automatically_derived]
        impl #impl_generics #krate::IterableMut for #struct_name #ty_generics #where_clause {
            fn field_at_mut(
                &mut self,
                #index_param: usize,
            ) -> ::core::option::Option<(&'static str, &mut dyn ::core::any::Any)> {
                match #index_param {
                    #(#arms_mut)*
                    _ => ::core::option::Option::None,
                }
            }
        }
    }
}

/// One `IterableAs<dyn Trait + '__i>` impl for a `to(dyn Trait)` request.
///
/// The fresh `'__i` lifetime intersects the struct's lifetimes through the
/// per-field `FieldTy: Trait + '__i` predicates, so this works for structs
/// borrowing non-`'static` data — unlike the `dyn Any` path.
fn expand_as_impl(
    input: &DeriveInput,
    krate: &Path,
    fields: &[FieldInfo],
    trait_object: &TypeTraitObject,
) -> TokenStream2 {
    let struct_name = &input.ident;
    let field_count = fields.len();
    let bounds = &trait_object.bounds;

    let (_, ty_generics, _) = input.generics.split_for_impl();

    let mut generics = input.generics.clone();
    generics.params.insert(0, syn::parse_quote!('__i));
    {
        let where_clause = generics.make_where_clause();
        let mut seen = HashSet::new();
        for field in fields {
            let ty = field.ty;
            if seen.insert(quote!(#ty).to_string()) {
                where_clause.predicates.push(syn::parse_quote_spanned!(
                    ty.span() => #ty: #bounds + '__i
                ));
            }
        }
    }
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let index_param = if field_count == 0 {
        quote!(_index)
    } else {
        quote!(index)
    };

    let arms = fields.iter().enumerate().map(|(i, field)| {
        let name = &field.name;
        let member = &field.member;
        quote! {
            #i => ::core::option::Option::Some(
                (#name, &self.#member as &(dyn #bounds + '__i)),
            ),
        }
    });

    quote! {
        #[automatically_derived]
        impl #impl_generics #krate::IterableAs<dyn #bounds + '__i>
            for #struct_name #ty_generics #where_clause
        {
            #[inline]
            fn field_count_as(&self) -> usize {
                #field_count
            }

            fn field_at_as(
                &self,
                #index_param: usize,
            ) -> ::core::option::Option<(&'static str, &(dyn #bounds + '__i))> {
                match #index_param {
                    #(#arms)*
                    _ => ::core::option::Option::None,
                }
            }

            #[inline]
            fn iter_as(&self) -> #krate::FieldIterAs<'_, dyn #bounds + '__i> {
                #krate::FieldIterAs::new(self)
            }
        }
    }
}

fn parse_container_attrs(attrs: &[syn::Attribute]) -> syn::Result<ContainerAttrs> {
    let mut out = ContainerAttrs {
        crate_path: None,
        to: Vec::new(),
        no_any: false,
    };
    for attr in attrs {
        if !attr.path().is_ident("iterable") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out.crate_path = Some(lit.parse()?);
                Ok(())
            } else if meta.path.is_ident("to") {
                let content;
                syn::parenthesized!(content in meta.input);
                let trait_object: TypeTraitObject = content.parse()?;
                validate_trait_object(&trait_object)?;
                out.to.push(trait_object);
                Ok(())
            } else if meta.path.is_ident("no_any") {
                out.no_any = true;
                Ok(())
            } else {
                Err(meta.error(
                    "unknown attribute; expected `to(dyn Trait)`, `no_any`, \
                     or `crate = \"...\"`",
                ))
            }
        })?;
    }
    Ok(out)
}

fn validate_trait_object(trait_object: &TypeTraitObject) -> syn::Result<()> {
    if trait_object.dyn_token.is_none() {
        return Err(syn::Error::new_spanned(
            trait_object,
            "write the trait object with `dyn`: `to(dyn Trait)`",
        ));
    }
    let mut trait_bounds = Vec::new();
    for bound in &trait_object.bounds {
        match bound {
            syn::TypeParamBound::Lifetime(lifetime) => {
                return Err(syn::Error::new_spanned(
                    lifetime,
                    "the lifetime of the trait object is inserted automatically; \
                     remove the explicit lifetime",
                ));
            }
            syn::TypeParamBound::Trait(bound) => trait_bounds.push(bound),
            _ => {}
        }
    }
    if let [only] = trait_bounds[..] {
        if is_any_path(&only.path) {
            return Err(syn::Error::new_spanned(
                trait_object,
                "`to(dyn Any)` is redundant: every `Iterable` already implements \
                 `IterableAs<dyn Any>`",
            ));
        }
    }
    Ok(())
}

/// Recognizes `Any`, `core::any::Any`, and `std::any::Any` (with or without a
/// leading `::`). A user trait genuinely named `Any` can be disambiguated by
/// path-qualifying it.
fn is_any_path(path: &Path) -> bool {
    let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    matches!(
        segments
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .as_slice(),
        ["Any"] | ["core", "any", "Any"] | ["std", "any", "Any"]
    )
}

struct FieldAttrs {
    skip: bool,
    rename: Option<String>,
}

fn parse_field_attrs(field: &syn::Field) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs {
        skip: false,
        rename: None,
    };
    for attr in &field.attrs {
        if !attr.path().is_ident("iterable") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
                Ok(())
            } else if meta.path.is_ident("rename") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out.rename = Some(lit.value());
                Ok(())
            } else {
                Err(meta.error("unknown field attribute; expected `skip` or `rename = \"...\"`"))
            }
        })?;
    }
    Ok(out)
}
