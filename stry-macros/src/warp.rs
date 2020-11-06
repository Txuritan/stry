use {
    proc_macro2::TokenStream,
    syn::{
        punctuated::Punctuated, Attribute, FnArg, Generics, Ident, ItemFn, LitStr, Pat, PatIdent,
        PatType, Path, Signature, Type, TypePath,
    },
};

pub(crate) enum Method {
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
}

struct Triplet<K, S, V> {
    key: K,
    _sep: S,
    value: V,
}

impl<K, S, V> syn::parse::Parse for Triplet<K, S, V>
where
    K: syn::parse::Parse,
    S: syn::parse::Parse,
    V: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _sep: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct UrlParams<'u> {
    token: &'u LitStr,
    items: Vec<UrlParam<'u>>,
}

enum UrlParam<'u> {
    Chunk { value: &'u str },
    Param { key: &'u str },
}

struct FnParam<'p> {
    token: &'p FnArg,

    key: &'p Ident,
    typ: &'p Path,

    kind: Option<FnParamKind>,
}

enum FnParamKind {
    Data,
    Form,
    Header { header: LitStr },
    Json,
    Query,
}

fn is_attr(attr: &Attribute) -> bool {
    for name in &["data", "form", "header", "json", "query"] {
        if attr.path.is_ident(name) {
            return true;
        }
    }

    false
}

pub(crate) fn route(method: Method, path: LitStr, item: ItemFn) -> proc_macro::TokenStream {
    // Rebuilt the original function to be placed in wrapper function's body
    let (asyncness, inner) = match build_inner(&item) {
        Ok(pair) => pair,
        Err(stream) => return proc_macro::TokenStream::from(stream),
    };

    // Extract the function parameters for the wrapper function and remove the old attributes
    let wrapper_inputs = item
        .sig
        .inputs
        .clone()
        .into_iter()
        .filter(|input| match input {
            FnArg::Receiver(_) => false,
            FnArg::Typed(PatType { attrs, .. }) => attrs.iter().any(|a| a.path.is_ident("data")),
        })
        .map(|input| match input {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(PatType {
                attrs,
                pat,
                colon_token,
                ty,
            }) => FnArg::Typed(PatType {
                attrs: attrs
                    .into_iter()
                    .partition(is_attr)
                    .1,
                pat,
                colon_token,
                ty,
            }),
        })
        .collect::<Vec<_>>();

    // Build the wrap filter with the correct handler closure
    let filter = match build_filter(method, &path, &item, asyncness, &wrapper_inputs) {
        Ok(stream) => stream,
        Err(stream) => return proc_macro::TokenStream::from(stream),
    };

    let vis = &item.vis;
    let name = &item.sig.ident;

    proc_macro::TokenStream::from(quote::quote! {
        #vis fn #name( #( #wrapper_inputs ),* ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + std::clone::Clone {
            #inner

            #[allow(unused_imports)]
            {
                use warp::Filter;

                #filter
            }
        }
    })
}

fn build_filter(method: Method, path: &LitStr, item: &ItemFn, asyncness: bool, wrapper_inputs: &[FnArg]) -> Result<TokenStream, TokenStream> {
    let path_value = path.value();

    // Convert additional function attributes to warp filters
    let attrs_stream = parse_attrs(&item.attrs);

    // Create the routing filters and closure inputs
    let (closure, routing) = {
        let fun_params = parse_params(&item.sig.inputs)?;

        let url_params = parse_url(path, &path_value);

        build_filters_and_inputs(&url_params, &fun_params)?
    };

    // Reference the data types to clone them
    let data = wrapper_inputs.iter().map(|input| match input {
        FnArg::Receiver(_) => unreachable!(),
        FnArg::Typed(PatType { pat, .. }) => pat,
    });

    // Extract the function parameter names
    let fn_inputs = item
        .sig
        .inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(PatType { pat, .. }) => pat,
        })
        .collect::<Vec<_>>();

    // Async functions use `and_then` while sync functions use `map`
    // Select and build the closure for the right one
    let map = if asyncness {
        quote::quote! { .and_then(|#closure| {
            #( let #data = #data.clone(); )*

            async move {
                inner(#( #fn_inputs ),*).await
            }
        }) }
    } else {
        quote::quote! { .map(|#closure| {
            #( let #data = #data.clone(); )*

            inner(#( #fn_inputs ),*)
        }) }
    };

    // Select the correct filter base
    let method_handler = match method {
        Method::Delete => quote::quote! { delete() },
        Method::Get => quote::quote! { get() },
        Method::Head => quote::quote! { head() },
        Method::Options => quote::quote! { options() },
        Method::Patch => quote::quote! { patch() },
        Method::Post => quote::quote! { post() },
        Method::Put => quote::quote! { get() },
    };

    // Automatically imports warp's Filter trait
    Ok(quote::quote! {
        warp::filters::method::#method_handler
        #attrs_stream
        #routing
        .and(warp::filters::path::end())
        #map
    })
}

// Rebuilt the original function to be placed in wrapper function's body
fn build_inner(item: &ItemFn) -> Result<(bool, TokenStream), TokenStream> {
    let block = &item.block;

    let Signature {
        asyncness,
        unsafety,
        generics,
        output,
        inputs,
        ..
    } = &item.sig;

    // The custom attributes need to be removed be I rebuild the function
    let new_inputs: Punctuated<FnArg, syn::Token![,]> = clean_inputs(inputs)?;

    let Generics {
        params,
        where_clause,
        ..
    } = generics;

    Ok((
        asyncness.is_some(),
        quote::quote! {
            #[inline(always)]
            #unsafety #asyncness fn inner #params ( #new_inputs ) #output #where_clause #block
        },
    ))
}

// Remove any custom attributes from function inputs
fn clean_inputs(
    inputs: &Punctuated<FnArg, syn::Token![,]>,
) -> Result<Punctuated<FnArg, syn::Token![,]>, TokenStream> {
    let mut cleaned: Punctuated<FnArg, syn::Token![,]> = inputs.clone();

    for mut pair in cleaned.pairs_mut() {
        let value = pair.value_mut();

        match value {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    value,
                    "Routing macro does not allow for self referencing functions",
                )
                .to_compile_error())
            }
            FnArg::Typed(PatType { attrs, .. }) => {
                *attrs = attrs
                    .clone()
                    .into_iter()
                    .partition(is_attr)
                    .1;
            }
        }
    }

    Ok(cleaned)
}

// Create the actually routing filters
fn build_filters_and_inputs(
    url_params: &UrlParams<'_>,
    fun_params: &[FnParam<'_>],
) -> Result<(TokenStream, TokenStream), TokenStream> {
    let mut closure = Vec::new();
    let mut routing = Vec::new();

    // Convert URL chunks into warp filters
    for param in &url_params.items {
        match param {
            UrlParam::Chunk { value } => {
                routing.push(quote::quote! { and(warp::filters::path::path(#value)) });
            }
            UrlParam::Param { key } => {
                // Get the URL param type
                let param = fun_params.iter().find(|param| param.key == *key);

                match param {
                    Some(FnParam { typ, .. }) => {
                        let ident = quote::format_ident!("{}", key);

                        closure.push(quote::quote! { #ident: #typ });
                        routing.push(quote::quote! { and(warp::filters::path::param::<#typ>()) });
                    }
                    None => {
                        let spanned = syn::Error::new_spanned(url_params.token, "Unable to find url param type, make sure the param has a matching function parameter");

                        return Err(spanned.to_compile_error());
                    }
                }
            }
        }
    }

    // Data filters go at the end of the filter, so I need to split them
    let (data, others): (Vec<&FnParam<'_>>, Vec<&FnParam<'_>>) = fun_params
        .iter()
        .partition(|p| matches!(p.kind, Some(FnParamKind::Data)));

    // Convert non data attributes to warp filters
    for FnParam {
        token,
        key,
        typ,
        kind,
    } in others
    {
        let stream = match kind {
            Some(FnParamKind::Data) => {
                let spanned = syn::Error::new_spanned(token, "warp-macros bug: This should not happen, invalid partitioned enum variant: non data");

                return Err(spanned.to_compile_error());
            }
            Some(FnParamKind::Form) => {
                quote::quote! { and(warp::filters::body::form::<#typ>()) }
            }
            Some(FnParamKind::Header { header }) => {
                quote::quote! { and(warp::filters::header::header::<#typ>(#header)) }
            }
            Some(FnParamKind::Json) => {
                quote::quote! { and(warp::filters::body::json::<#typ>()) }
            }
            Some(FnParamKind::Query) => {
                quote::quote! { and(warp::filters::query::query::<#typ>()) }
            }
            None => continue,
        };

        closure.push(quote::quote! { #key: #typ });
        routing.push(stream);
    }

    // Convert data attributes to warp filters
    for FnParam {
        token,
        key,
        typ,
        kind,
    } in data
    {
        let stream = match kind {
            Some(FnParamKind::Data) => {
                quote::quote! { and(warp::filters::any::any().map(move || #key.clone())) }
            }
            Some(FnParamKind::Form) | Some(FnParamKind::Header { .. }) | Some(FnParamKind::Json) | Some(FnParamKind::Query) => {
                let spanned = syn::Error::new_spanned(token, "warp-macros bug: This should not happen, invalid partitioned enum variant: data");

                return Err(spanned.to_compile_error());
            }
            None => continue,
        };

        closure.push(quote::quote! { #key: #typ });
        routing.push(stream);
    }

    Ok((
        quote::quote! { #( #closure ),* },
        quote::quote! { #( . #routing )* },
    ))
}

// Convert the route into chunks
// Empty will be handled as `/`
fn parse_url<'u>(token: &'u LitStr, url: &'u str) -> UrlParams<'u> {
    let items = if url.contains('{') {
        // Handles routes with arguments
        url.match_indices('{')
            .zip(url.match_indices('}'))
            .fold(
                (0, Vec::with_capacity(16)),
                |(last, mut parts), ((start, _), (end, _))| {
                    // Converts any route data between the last and the current argument
                    parts.extend(
                        (&url[last..start])
                            .split('/')
                            .filter(|p| !p.is_empty())
                            .map(|value| UrlParam::Chunk { value }),
                    );

                    // Grabs the name of the current argument
                    parts.push(UrlParam::Param {
                        key: &url[(start + 1)..end],
                    });

                    (end + 1, parts)
                },
            )
            .1
    } else {
        // Handles routes without arguments
        url.split('/')
            .filter(|p| !p.is_empty())
            .map(|value| UrlParam::Chunk { value })
            .collect()
    };

    UrlParams { token, items }
}

fn parse_attrs(attrs: &[Attribute]) -> TokenStream {
    let mut parts = Vec::new();

    for attr in attrs {
        let attr_path = &attr.path;

        if attr_path.is_ident("header") {
            // Attempt to parse the attribute's body
            let Triplet { key, value, .. }: Triplet<LitStr, syn::Token![:], LitStr> = match attr
                .parse_args()
            {
                Ok(value) => value,
                Err(err) => {
                    let mut spanned = syn::Error::new_spanned(
                        &attr.tokens,
                        r#"failed to parse attribute, example: #[header("Content-Type": "application/json")]"#,
                    );

                    spanned.combine(err);

                    return spanned.to_compile_error();
                }
            };

            parts.push(quote::quote! { and(warp::filters::header::exact(#key, #value)) });

            continue;
        }

        if attr_path.is_ident("body_size") {
            // Attempt to parse the attribute's body
            let Triplet { key, value, .. }: Triplet<Ident, syn::Token![=], LitStr> =
                match attr.parse_args() {
                    Ok(value) => value,
                    Err(err) => {
                        let mut spanned = syn::Error::new_spanned(
                            &attr.tokens,
                            r#"failed to parse attribute, example: #[body_size(max = "1024")]"#,
                        );

                        spanned.combine(err);

                        return spanned.to_compile_error();
                    }
                };

            if key == "max" {
                parts
                    .push(quote::quote! { and(warp::filters::body::content_length_limit(#value)) });
            } else {
                return syn::Error::new_spanned(
                    &attr.tokens,
                    format!(r#"failed to parse attribute, invalid type `{}`, example: #[body_size(max = "1024")]"#, key),
                ).to_compile_error();
            }

            continue;
        }

        // TODO: CORS
    }

    quote::quote! { #( . #parts )* }
}

fn parse_params<'p>(
    params: &'p Punctuated<FnArg, syn::Token![,]>,
) -> Result<Vec<FnParam<'p>>, TokenStream> {
    let mut mapped = Vec::with_capacity(params.len());

    for param in params {
        match param {
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                // Extract needed type data
                let key = match &**pat {
                    Pat::Ident(PatIdent { ident, .. }) => ident,
                    pat => {
                        return Err(syn::Error::new_spanned(
                            pat,
                            "Unsupported function argument pattern type",
                        )
                        .to_compile_error());
                    }
                };

                let typ = match &**ty {
                    Type::Path(TypePath { path, .. }) => path,
                    typ => {
                        return Err(syn::Error::new_spanned(
                            typ,
                            "Unsupported function argument type",
                        )
                        .to_compile_error());
                    }
                };

                let mut kind = None;

                // Check for supported attributes
                for attr in attrs {
                    let attr_path = &attr.path;

                    if attr_path.is_ident("data") {
                        kind = Some(FnParamKind::Data);

                        break;
                    }

                    if attr_path.is_ident("form") {
                        kind = Some(FnParamKind::Form);

                        break;
                    }

                    if attr_path.is_ident("header") {
                        let header = attr.parse_args().map_err(|err| err.to_compile_error())?;

                        kind = Some(FnParamKind::Header { header });

                        break;
                    }

                    if attr_path.is_ident("query") {
                        kind = Some(FnParamKind::Query);

                        break;
                    }

                    if attr_path.is_ident("json") {
                        kind = Some(FnParamKind::Json);

                        break;
                    }
                }

                mapped.push(FnParam {
                    token: param,

                    key,
                    typ,

                    kind,
                });
            }
            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    param,
                    "Routing macro does not allow for self referencing functions",
                )
                .to_compile_error());
            }
        }
    }

    Ok(mapped)
}
