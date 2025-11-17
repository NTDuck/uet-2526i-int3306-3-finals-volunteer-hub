mod verifiable;

#[proc_macro_derive(Verifiable, attributes(verifiable))]
pub fn derive_verifiable(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    self::verifiable::derive(tokens)
}
