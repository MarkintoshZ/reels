use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn_mid::{FnArg, ItemFn};

use crate::args::Args;

pub fn expand(args: Args, func: ItemFn) -> TokenStream {
    let methods = args
        .methods
        .iter()
        .map(|method| syn::Ident::new(method, Span::call_site()));
    let url_pattern = args.url.to_string();
    let vis = &func.vis;
    let ident = &func.sig.ident;
    let inputs = &func.sig.inputs;
    let arg_types: Vec<proc_macro2::TokenStream> = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(arg) => arg.ty.clone(),
            _ => unreachable!(),
        })
        .map(|ty| {
            let ty_str = quote! { #ty }.to_string();
            if ty_str == "& str" {
                quote! {
                    match captures.next() {
                        Some(SegmentPatternValue::Wildcard(v)) => v,
                        _ => return Err(reels_core::router::SegmentTypeMissmatch),
                    }
                }
            } else if ty_str == "Vec < & str >" {
                quote! {
                    match captures.next() {
                        Some(SegmentPatternValue::WildcardKleene(v)) => v,
                        _ => return Err(reels_core::router::SegmentTypeMissmatch),
                    }
                }
            } else {
                quote! {
                    match captures.next() {
                        Some(SegmentPatternValue::Wildcard(v)) =>
                            v.parse::<#ty>().map_err(|_| reels_core::router::SegmentTypeMissmatch)?,
                        _ => return Err(reels_core::router::SegmentTypeMissmatch),
                    }
                }
            }
        })
        .collect();

    let output = quote! {
        #vis fn #ident() -> (Vec<reels::http::Method>, &'static str, reels_core::router::HandlerFunc) {
            fn #ident(
                captures: reels_core::router::PathCapture,
                request: &reels_core::http::HttpRequest
            ) -> Result<reels_core::http::HttpResponse, reels_core::router::SegmentTypeMissmatch> {
                #func

                let mut captures = captures.into_iter();
                Ok(#ident(
                    #(#arg_types)*
                ))
            }

            (vec![#(reels::http::Method::#methods)*], #url_pattern, #ident)
        }
    };
    output.into()
}
