use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, ItemFn, LitStr, ReturnType, parse::Parser, parse_macro_input,
};

#[derive(Clone, Copy)]
enum ClassKind {
    Component,
    Resource,
    Asset,
    Value,
}

impl ClassKind {
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

struct ClassArgs {
    module: String,
    name: Option<String>,
    legacy_header: String,
    kind: ClassKind,
}

impl Default for ClassArgs {
    fn default() -> Self {
        Self {
            module: "trinity".to_owned(),
            name: None,
            legacy_header: String::new(),
            kind: ClassKind::Component,
        }
    }
}

fn class_args(input: &DeriveInput) -> syn::Result<ClassArgs> {
    let mut args = ClassArgs::default();
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
                args.legacy_header = meta.value()?.parse::<LitStr>()?.value();
            } else if meta.path.is_ident("kind") {
                args.kind = ClassKind::parse(&meta.value()?.parse::<LitStr>()?.value())?;
            }
            Ok(())
        })?;
    }
    Ok(args)
}

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

fn doc_string(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| match &attr.meta {
            syn::Meta::NameValue(value) => match &value.value {
                syn::Expr::Lit(expr) => match &expr.lit {
                    syn::Lit::Str(value) => Some(value.value().trim().to_owned()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[proc_macro_derive(TrinityClass, attributes(trinity))]
pub fn derive_trinity_class(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_trinity_class(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_trinity_class(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let args = class_args(input)?;
    let module = LitStr::new(&args.module, ident.span());
    let carbon_name = LitStr::new(
        args.name.as_deref().unwrap_or(&ident.to_string()),
        ident.span(),
    );
    let rust_name = LitStr::new(&ident.to_string(), ident.span());
    let legacy_header = LitStr::new(&args.legacy_header, ident.span());
    let doc = LitStr::new(&doc_string(&input.attrs), ident.span());
    let kind = match args.kind {
        ClassKind::Component => quote!(::trinity_meta::TrinityClassKind::Component),
        ClassKind::Resource => quote!(::trinity_meta::TrinityClassKind::Resource),
        ClassKind::Asset => quote!(::trinity_meta::TrinityClassKind::Asset),
        ClassKind::Value => quote!(::trinity_meta::TrinityClassKind::Value),
    };

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .filter_map(|field| field.ident.as_ref().map(|ident| (ident, field)))
                .map(|(field_ident, field)| {
                    let flags = field_flags(&field.attrs)?;
                    if flags.skip {
                        return Ok(None);
                    }
                    let name = LitStr::new(&field_ident.to_string(), field_ident.span());
                    let read = flags.read;
                    let write = flags.write;
                    let persist = flags.persist;
                    Ok(Some(quote! {
                        ::trinity_meta::TrinityFieldInfo::new(#name, #read, #write, #persist)
                    }))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            Fields::Unnamed(_) | Fields::Unit => Vec::new(),
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
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("legacy") {
            legacy = Some(meta.value()?.parse::<LitStr>()?.value());
            return Ok(());
        }
        if meta.path.is_ident("verification") {
            verification = meta.value()?.parse::<LitStr>()?.value();
            return Ok(());
        }
        Err(meta.error("supported options: legacy, verification"))
    });
    parser.parse2(tokens)?;
    Ok(TestArgs {
        legacy: legacy.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "legacy = \"...\" is required",
            )
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
    match expand_test(args, function) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_test(args: TestArgs, function: ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    if !function.sig.inputs.is_empty() || !matches!(function.sig.output, ReturnType::Default) {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "#[trinity_test] requires fn() with no explicit return type",
        ));
    }

    let attrs = &function.attrs;
    let vis = &function.vis;
    let name = &function.sig.ident;
    let body_name = format_ident!("__trinity_test_body_{name}");
    let descriptor_name = format_ident!("__TRINITY_TEST_{}", name.to_string().to_ascii_uppercase());
    let block = &function.block;
    let legacy = LitStr::new(&args.legacy, name.span());
    let rust_test = LitStr::new(&name.to_string(), name.span());
    let verification = match args.verification.as_str() {
        "kani" => quote!(::trinity_meta::VerificationClass::Kani),
        "native" => quote!(::trinity_meta::VerificationClass::Native),
        "render-device" => quote!(::trinity_meta::VerificationClass::RenderDevice),
        "screenshot" => quote!(::trinity_meta::VerificationClass::Screenshot),
        "integration" => quote!(::trinity_meta::VerificationClass::Integration),
        "unmapped" => quote!(::trinity_meta::VerificationClass::Unmapped),
        _ => {
            return Err(syn::Error::new(
                name.span(),
                "verification must be kani, native, render-device, screenshot, integration, or unmapped",
            ));
        }
    };

    Ok(quote! {
        #(#attrs)*
        #[test]
        #vis fn #name() {
            #body_name();
        }

        #[doc(hidden)]
        pub fn #body_name() #block

        #[doc(hidden)]
        pub const #descriptor_name: ::trinity_meta::TrinityTestDescriptor =
            ::trinity_meta::TrinityTestDescriptor::new(#legacy, #rust_test, #verification);
    })
}
