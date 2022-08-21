#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;

mod args;
mod expand;

use args::Args;
use expand::expand;

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
    // TODO: add parameter that specifies a list of http methods to handle
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    if args.methods.is_empty() {
        args.methods = vec![
            "Get".to_string(),
            "Put".to_string(),
            "Head".to_string(),
            "Post".to_string(),
            "Patch".to_string(),
            "Trace".to_string(),
            "Delete".to_string(),
            "Connect".to_string(),
            "Options".to_string(),
        ];
    }

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP get request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[get("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn get(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP put request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[put("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn put(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP head request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[head("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn head(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP post request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[post("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn post(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP patch request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[patch("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn patch(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP trace request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[trace("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn trace(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP delete request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[delete("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn delete(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP connect request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[connect("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn connect(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

/// Define HTTP option request handler with typed url path capture(s)
///
/// Examples
/// ```ignore
/// #[option("/users/<username>")]
/// fn users(username: &str) -> HttpResponse {
///     HttpResponse::builder()
///         .status(StatusCode::OK)
///         .body(format!("Hi, {}", username).to_owned())
///         .finalize()
/// }
/// ```
#[proc_macro_attribute]
pub fn option(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut args: Args = match syn::parse(args) {
        Ok(args) => args,
        Err(e) => return token_stream_with_error(item, e),
    };
    args.methods.push("Get".to_string());

    match syn::parse(item.clone()) {
        Ok(it) => expand(args, it),
        Err(e) => token_stream_with_error(item, e),
    }
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}
