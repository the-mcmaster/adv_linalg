use proc_macro::TokenStream;

mod prealloc;

#[proc_macro_attribute]
pub fn prealloc(input: TokenStream, _: TokenStream) -> TokenStream {
    todo!()
}