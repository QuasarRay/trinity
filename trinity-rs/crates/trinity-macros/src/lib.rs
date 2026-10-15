use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, ItemFn, LitStr, ReturnType, parse::Parser, parse_macro_input,
};

#[derive(Clone, Copy)]
enum Kind {
    Component,
    Resource,
    Asset,
    Value,
}

impl Kind {
    fn parse(value: &str) -> syn::Result<Self> {
        match value {
            "component" => Ok(Self::Component),
            "resource" => Ok(Self::Resource),
            "asset" => Ok(Self::Asset),
            "value" => Ok(Self::Value),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "kind must be component, resource, asset, or value",
            )),
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
struct FieldFlags {
    read: bool,
    write: bool,
    persist: bool,
    skip: bool,
}

fn field_flags(attrs: &[syn::Attribute]) -> syn::Result<FieldFlags> {
    let mut flags = FieldFlags::default();
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("trinity")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("read") {
                flags.read = true;
            } else if meta.path.is_ident("write") {
                flags.write = true;
            } else if meta.path.is_ident("persist") {
                flags.persist = true;
            } else if meta.path.is_ident("skip") {
                flags.skip = true;
            }
            Ok(())
        })?;
    }
    Ok(flags)
}

fn docs(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| match &attr.meta {
            syn::Meta::NameValue(value) => match &value.value {
                syn::Expr::Lit(expr) => match &expr.lit {
                    syn::Lit::Str(doc) => Some(doc.value().trim().to_owned()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .filter(|doc| !doc.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

struct ClassArgs {
    module: String,
    name: Option<String>,
    legacy: String,
    kind: Kind,
}

fn class_args(input: &DeriveInput) -> syn::Result<ClassArgs> {
    let mut args = ClassArgs {
        module: "trinity".into(),
        name: None,
        legacy: String::new(),
        kind: Kind::Component,
    };
    for attr in input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("trinity"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("module") {
                args.module = meta.value()?.parse::<LitStr>()?.value();
            } else if meta.path.is_ident("name") {
                args.name = Some(meta.value()?.parse::<LitStr>()?.value());
            } else if meta.path.is_ident("legacy_header") {
                args.legacy = meta.value()?.parse::<LitStr>()?.value();
            } else if meta.path.is_ident("kind") {
                args.kind = Kind::parse(&meta.value()?.parse::<LitStr>()?.value())?;
            }
            Ok(())
        })?;
    }
    Ok(args)
}

#[proc_macro_derive(TrinityClass, attributes(trinity))]
pub fn derive_trinity_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_class(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_class(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let args = class_args(input)?;
    let module = LitStr::new(&args.module, ident.span());
    let owned_name = ident.to_string();
    let carbon_name = LitStr::new(args.name.as_deref().unwrap_or(&owned_name), ident.span());
    let rust_name = LitStr::new(&owned_name, ident.span());
    let legacy_header = LitStr::new(&args.legacy, ident.span());
    let doc = LitStr::new(&docs(&input.attrs), ident.span());
    let kind = match args.kind {
        Kind::Component => quote!(::trinity_meta::TrinityClassKind::Component),
        Kind::Resource => quote!(::trinity_meta::TrinityClassKind::Resource),
        Kind::Asset => quote!(::trinity_meta::TrinityClassKind::Asset),
        Kind::Value => quote!(::trinity_meta::TrinityClassKind::Value),
    };

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named_fields) => {
                let mut infos = Vec::new();
                for field in &named_fields.named {
                    let Some(field_ident) = &field.ident else {
                        continue;
                    };
                    let flags = field_flags(&field.attrs)?;
                    if flags.skip {
                        continue;
                    }
                    let field_name = LitStr::new(&field_ident.to_string(), field_ident.span());
                    let read = flags.read;
                    let write = flags.write;
                    let persist = flags.persist;
                    infos.push(quote!(
                        ::trinity_meta::TrinityFieldInfo::new(
                            #field_name,
                            #read,
                            #write,
                            #persist
                        )
                    ));
                }
                infos
            }
            _ => Vec::new(),
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "TrinityClass can only be derived for structs",
            ));
        }
    };

    Ok(quote! {
        impl ::trinity_meta::TrinityClass for #ident {
            const CARBON_MODULE: &'static str = #module;
            const CARBON_NAME: &'static str = #carbon_name;

            fn trinity_class_info() -> ::trinity_meta::TrinityClassInfo {
                ::trinity_meta::TrinityClassInfo {
                    module: #module.to_owned(),
                    carbon_name: #carbon_name.to_owned(),
                    rust_name: #rust_name.to_owned(),
                    legacy_header: #legacy_header.to_owned(),
                    doc: #doc.to_owned(),
                    kind: #kind,
                    fields: vec![#(#fields),*],
                }
            }
        }
    })
}

struct TestArgs {
    legacy: String,
    verification: String,
}

fn parse_test_args(tokens: proc_macro2::TokenStream) -> syn::Result<TestArgs> {
    let mut legacy = None;
    let mut verification = "kani".to_owned();
    syn::meta::parser(|meta| {
        if meta.path.is_ident("legacy") {
            legacy = Some(meta.value()?.parse::<LitStr>()?.value());
            Ok(())
        } else if meta.path.is_ident("verification") {
            verification = meta.value()?.parse::<LitStr>()?.value();
            Ok(())
        } else {
            Err(meta.error("supported options: legacy, verification"))
        }
    })
    .parse2(tokens)?;
    Ok(TestArgs {
        legacy: legacy.ok_or_else(|| {
            syn::Error::new(proc_macro2::Span::call_site(), "legacy is required")
        })?,
        verification,
    })
}

#[proc_macro_attribute]
pub fn trinity_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match parse_test_args(attr.into()) {
        Ok(args) => args,
        Err(error) => return error.into_compile_error().into(),
    };
    let function = parse_macro_input!(item as ItemFn);
    if !function.sig.inputs.is_empty() || !matches!(function.sig.output, ReturnType::Default) {
        return syn::Error::new_spanned(&function.sig, "#[trinity_test] requires fn()")
            .into_compile_error()
            .into();
    }

    let attrs = &function.attrs;
    let visibility = &function.vis;
    let name = &function.sig.ident;
    let body_name = format_ident!("__trinity_test_body_{name}");
    let descriptor_name =
        format_ident!("__TRINITY_TEST_{}", name.to_string().to_ascii_uppercase());
    let block = &function.block;
    let legacy = LitStr::new(&args.legacy, name.span());
    let rust_name = LitStr::new(&name.to_string(), name.span());
    let verification = match args.verification.as_str() {
        "kani" => quote!(::trinity_meta::VerificationClass::Kani),
        "native" => quote!(::trinity_meta::VerificationClass::Native),
        "render-device" => quote!(::trinity_meta::VerificationClass::RenderDevice),
        "screenshot" => quote!(::trinity_meta::VerificationClass::Screenshot),
        "integration" => quote!(::trinity_meta::VerificationClass::Integration),
        _ => quote!(::trinity_meta::VerificationClass::Unmapped),
    };

    quote! {
        #(#attrs)*
        #[test]
        #visibility fn #name() {
            #body_name();
        }

        #[doc(hidden)]
        pub fn #body_name() #block

        #[doc(hidden)]
        pub const #descriptor_name: ::trinity_meta::TrinityTestDescriptor =
            ::trinity_meta::TrinityTestDescriptor::new(#legacy, #rust_name, #verification);
    }
    .into()
}
