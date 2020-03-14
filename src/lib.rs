extern crate proc_macro;
use self::proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use syn::{parse_macro_input,
          AttributeArgs,
          Item,
          Meta,
          MetaList,
          NestedMeta,
          MetaNameValue,
          Lit,
          LitStr,
};
use quote::{quote, format_ident};
use inflector::Inflector;

// Example:
// #[match_enum_from_ffi(prefix="ffi::idevice_error_t_IDEVICE_E_")]
// #[derive(Debug, PartialEq)]
// pub enum DeviceError {
//     #[undefined]
//     Undefined(i32), // this is used to handle when enum value returned from C has been changed somehow
//     #[ffi_enum(suffix="SUCCESS")]
//     #[success]
//     Success,
//     #[ffi_enum(suffix="INVALID_ARG")]
//     InvalidArg,
//     #[ffi_enum(suffix="UNKNOWN_ERROR")]
//     UnknownError,
//     #[ffi_enum(suffix="NO_DEVICE")]
//     NoDevice,
//     #[ffi_enum(suffix="NOT_ENOUGH_DATA")]
//     NotEnoughData,
//     #[ffi_enum(suffix="SSL_ERROR")]
//     SslError,
//     #[ffi_enum(suffix="TIMEOUT")]
//     Timeout,
// }

#[proc_macro_attribute]
pub fn match_enum_from_ffi(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ast = parse_macro_input!(attr as AttributeArgs);
    // eprintln!("attr_ast: {:#?}", attr_ast);

    // to extract (prefix="ffi::idevice_error_t_IDEVICE_E_")
    let mut prefix:String = String::new();
    for meta in attr_ast.iter() {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue{path, lit: Lit::Str(lit), ..})) = meta {
            if let Some(ident) = path.get_ident() {
                if ident == "prefix" {
                    // eprintln!("name value: {:#?}:{:#?}", ident, lit.value());
                    prefix = lit.value();
                }
            }
        }
    }
    let prefix = prefix;

    let ast = parse_macro_input!(item as Item);
    // eprintln!("ast: {:#?}", ast);
    
    let mut out_tokenstream = Vec::<TokenStream2>::new();
    let mut wildcard_arm : Option<proc_macro2::TokenStream> = None;

    
    let mut out_ast = ast.clone();
    if let Item::Enum(out_item) = &mut out_ast {
        let enum_ident = out_item.ident.clone(); 
        let enum_name = enum_ident.to_string();
        // out_item.attrs.clear();

        let variants = &mut out_item.variants;
        for variant in variants.iter_mut() {
            // eprintln!("variant: {:#?}", variant);
            let variant_ident = variant.ident.clone();
            // extract #[ffi_enum(suffix="SUCCESS", success)]
            let nested_array = variant.attrs
                .iter()
                .filter_map(|attr| { // extract ffi_enum
                    if let Ok(
                        Meta::List(
                            MetaList {
                                path, nested, ..
                            })) = attr.parse_meta() {
                        if let Some(path_ident) = path.get_ident() { 
                           // ffi_enum, attribute name
                            if path_ident == "ffi_enum" {
                                return Some(nested);
                            }
                        }
                    }
                    None
                });

            // I need to put wildcard arm to be last one
            for nested in nested_array {
                let mut lit_value: Option<LitStr> = None;
                let mut is_undefined = false;

                for meta in nested {
                    // eprintln!("{:#?}", meta);
                    if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) = meta {
                        if let Some(ident) = path.get_ident() {
                            match ident.to_string().as_ref() {
                                "suffix" => {
                                    if let Lit::Str(lit) = lit {
                                        lit_value = Some(lit);
                                    }
                                },
                                _ => (),
                            }
                        }
                    }
                    else if let NestedMeta::Meta(Meta::Path(path)) = meta {
                        if let Some(ident) = path.get_ident() {
                            match ident.to_string().as_ref() {
                                "undefined" => {
                                    // eprintln!("is_undefined - {:?}", variant_ident.to_string());
                                    is_undefined = true;
                                },
                                _ => (),
                            }
                        }
                    }
                }
                if let Some(lit) = lit_value {
                    let full_str = format!("{}{}", prefix, lit.value());
                    if let Ok(ty) = syn::parse_str::<syn::Path>(full_str.as_ref()) {
                        // eprintln!("{:#?}", ty);
                        let q = quote! {
                            #ty => #enum_ident::#variant_ident
                        };
                        out_tokenstream.push(q);
                    }
                }
                if is_undefined {
                    if wildcard_arm.is_none() {
                        let q = quote! {
                            (other) => #enum_ident::#variant_ident(other)
                        };

                        // eprintln!("ts: {}", q.to_string());
                        wildcard_arm = Some(q);
                    }
                }
            }

            // to remove attributes
            variant.attrs.clear();
        }

        if let Some(q) = wildcard_arm {
            out_tokenstream.push(q);
        }
        // eprintln!("ts: {:#?}", out_tokenstream);
        let match_macro_ident = format_ident!("match_{}", enum_name.to_snake_case());
        // eprintln!("match_name: {}", match_macro_ident);

        /*
    match ffi::idevice_error_t_IDEVICE_E_SUCCESS {
        ffi::idevice_error_t_IDEVICE_E_SUCCESS => DeviceError::Success,
        ffi::idevice_error_t_IDEVICE_E_INVALID_ARG => DeviceError::InvalidArg,
        ffi::idevice_error_t_IDEVICE_E_UNKNOWN_ERROR => DeviceError::UnknownError,
        ffi::idevice_error_t_IDEVICE_E_NO_DEVICE => DeviceError::NoDevice,
        ffi::idevice_error_t_IDEVICE_E_NOT_ENOUGH_DATA => DeviceError::NotEnoughData,
        ffi::idevice_error_t_IDEVICE_E_SSL_ERROR => DeviceError::SslError,
        ffi::idevice_error_t_IDEVICE_E_TIMEOUT => DeviceError::Timeout,
        _(other) => DeviceError::Undefined(other),
    }*/
        
        let expanded = quote! {
             #out_ast

            #[macro_export]

            macro_rules! #match_macro_ident {
                ($e:expr) => {
                    match $e {
                        #(#out_tokenstream), *
                    }
                }
            }

        };
        // eprintln!("output: {}", expanded.to_string());
        return TokenStream::from(expanded);
    }
    else {
        let expanded = syn::Error::new(proc_macro2::Span::call_site(), "expected enum").to_compile_error();
        return TokenStream::from(expanded);
    }
}
