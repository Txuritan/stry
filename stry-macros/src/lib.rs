mod boxed;

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
