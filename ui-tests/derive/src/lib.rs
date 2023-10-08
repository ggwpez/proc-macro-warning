//! TESTING ONLY - DO NOT USE.

use proc_macro::TokenStream;
use quote::ToTokens;
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

	let warning = proc_macro_warning::Warning::new_deprecated("test").old("foo").new("bar");
	let warning = if span { warning.span(input.span()) } else { warning }.build_or_panic();

	warning.into_token_stream().into()
}

#[proc_macro_derive(DeprecatedRaw)]
pub fn deprecated_raw(input: TokenStream) -> TokenStream {
	impl_dep_raw(input)
}

fn impl_dep_raw(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	let warning = proc_macro_warning::FormattedWarning::new_deprecated(
            "VeryOldStuff",
            "\nMy message do noooooooooooooooooooooooooooooot formaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaat
or chaaaaaaaaaaaaange this, also no line breaks please ;)
other veryyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy looooooooooooooooooooooong lineeeeeeeeeeeeeeee",
            input.span(),
        );

	warning.into_token_stream().into()
}
