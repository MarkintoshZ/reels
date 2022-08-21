#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

use syn_mid::{FnArg, ItemFn};

use reels_url_pattern::UrlPattern;

/// Define HTTP request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[route("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn route(args: TokenStream, item: TokenStream) -> TokenStream {
    let args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

struct Args {
    url: UrlPattern,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: syn::LitStr = input.parse()?;
        let url = UrlPattern::parse(path.suffix())
            .map_err(|e| syn::Error::new(path.span(), "Not a valid url pattern"))?;
        Ok(Self { url })
    }
}

fn expand(args: Args, func: ItemFn) -> TokenStream {
    let attrs = &func.attrs;
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
        #(#attrs)*
        #vis fn #ident(
            captures: reels_core::router::PathCapture,
            request: &reels_core::http::HttpRequest
        ) -> Result<reels_core::http::HttpResponse, reels_core::router::SegmentTypeMissmatch> {
            #func

            let mut captures = captures.into_iter();
            Ok(#ident(
                #(#arg_types)*
            ))
        }
    };
    println!("{}", output);
    output.into()
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}
