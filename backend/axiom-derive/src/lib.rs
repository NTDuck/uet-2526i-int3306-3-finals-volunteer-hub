mod serializable_error;
mod verifiable;

/// Assumes: **(1)** the target is a tuple struct containing exactly one field typed `::axiom::string::String`; **(2)** `bon` and `thiserror` are within scope.
#[proc_macro_derive(Verifiable, attributes(verifiable))]
pub fn derive_verifiable(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    self::verifiable::derive(tokens)
}

/// Assumes: **(1)** the target is an owned, non-variadic enum; **(2)** `serde` and `thiserror` are within scope.
#[proc_macro_derive(SerializableError, attributes(error))]
pub fn derive_serializable_error(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    self::serializable_error::derive(tokens)
}
