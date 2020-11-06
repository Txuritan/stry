mod boxed;
mod warp;

#[proc_macro_attribute]
pub fn box_async(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(attr as boxed::Args);
    let mut item = syn::parse_macro_input!(item as boxed::Item);

    boxed::expand(&mut item, args.local);

    let stream = proc_macro::TokenStream::from(quote::quote!(#item));

    if false {
        panic!("{}", stream.to_string())
    } else {
        stream
    }
}

macro_rules! handler {
    ($( $name:ident => $method:expr, )*) => {
        $(
            #[proc_macro_attribute]
            pub fn $name(
                attr: proc_macro::TokenStream,
                body: proc_macro::TokenStream,
            ) -> proc_macro::TokenStream {
                $crate::warp::route(
                    $method,
                    syn::parse_macro_input!(attr as syn::LitStr),
                    syn::parse_macro_input!(body as syn::ItemFn),
                )
            }
        )*
    };
}

handler! {
    delete => crate::warp::Method::Delete,
    get => crate::warp::Method::Get,
    head => crate::warp::Method::Head,
    options => crate::warp::Method::Options,
    patch => crate::warp::Method::Patch,
    post => crate::warp::Method::Post,
    put => crate::warp::Method::Put,
}
