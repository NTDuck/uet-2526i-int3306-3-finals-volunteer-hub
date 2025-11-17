pub fn derive(tokens: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
    use ::darling::FromDeriveInput as _;
    use ::quote::ToTokens as _;

    let tokens = ::proc_macro2::TokenStream::from(tokens);

    let input = match ::syn::parse2(tokens) {
        ::core::result::Result::Ok(input) => input,
        ::core::result::Result::Err(error) => return error.into_compile_error().into(),
    };

    let input = match Verifiable::from_derive_input(&input) {
        ::core::result::Result::Ok(input) => input,
        ::core::result::Result::Err(error) => return error.write_errors().into(),
    };

    input.into_token_stream().into()
}

#[derive(::darling::FromDeriveInput)]
#[darling(attributes(verifiable), supports(struct_newtype))]
struct Verifiable {
    ident: ::syn::Ident,
    vis: ::syn::Visibility,
    generics: ::syn::Generics,

    regex: ::syn::LitStr,
    error: ::core::option::Option<::syn::LitStr>,
}

impl ::quote::ToTokens for Verifiable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use ::heck::ToTitleCase as _;

        let Self { ident, vis, generics, regex, error } = self;
        
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let error = match error {
            ::core::option::Option::Some(error) => error.value(),
            ::core::option::Option::None => ::std::format!(
                "Invalid {} format: does not match `{}`",
                ident.to_string().to_title_case(),
                regex.value().replace('{', "{{").replace('}', "}}"),
            )
        };

        let error_ident = ::quote::format_ident!("{ident}BuilderError");

        tokens.extend(::quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #vis fn new(value: ::axiom::aliases::string::String) -> ::core::result::Result<Self, #error_ident> {
                    Self::builder().value(value).build()
                }
            }

            #[::bon::bon]
            impl #impl_generics #ident #ty_generics #where_clause {
                #[builder(on(_, into))]
                #vis fn new(value: ::axiom::aliases::string::String) -> ::core::result::Result<Self, #error_ident> {
                    let value = normalize(value);
                    return validate(value).map(Self);

                    fn normalize(value: ::axiom::aliases::string::String) -> ::axiom::aliases::string::String {
                        let trimmed = value.trim();

                        if trimmed.len() == value.len() && !trimmed.chars().any(|char| char.is_control())
                        {
                            value
                        } else {
                            trimmed
                                .chars()
                                .filter(|char| !char.is_control())
                                .collect()
                        }
                    }

                    fn validate(value: ::axiom::aliases::string::String) -> ::core::result::Result<::axiom::aliases::string::String, #error_ident> {
                        let regex = ::axiom::macros::regex!(#regex);

                        if !regex.is_match(&value) {
                            ::core::result::Result::Err(#error_ident::InvalidFormat { value })
                        } else {
                            ::core::result::Result::Ok(value)
                        }
                    }
                }
            }

            impl #impl_generics ::core::ops::Deref for #ident #ty_generics #where_clause {
                type Target = ::axiom::aliases::string::String;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            #[derive(::core::fmt::Debug, ::core::clone::Clone, ::thiserror::Error)]
            #vis enum #error_ident {
                #[error(#error)]
                InvalidFormat { value: ::axiom::aliases::string::String },
            }
        });
    }
}