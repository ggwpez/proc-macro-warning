//! TESTING ONLY - DO NOT USE.

use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[proc_macro_derive(Deprecated)]
pub fn deprecated(input: TokenStream) -> TokenStream {
    impl_dep(input, true)
}

#[proc_macro_derive(DeprecatedNoSpan)]
pub fn deprecated2(input: TokenStream) -> TokenStream {
    impl_dep(input, false)
}

fn impl_dep(input: TokenStream, span: bool) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let warning = proc_macro_warning::Warning::new_deprecated("test")
        .old("foo").new("bar");
    let warning = if span {
        warning.span(input.span())
    } else {
        warning
    }.build();

    warning.into_token_stream().into()
}
