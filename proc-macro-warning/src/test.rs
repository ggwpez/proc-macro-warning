/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: (GPL-3.0 or Apache-2.0)
 */

#![cfg(test)]

use quote::quote;

use super::*;

#[test]
fn example_works() {
	let warning = Warning::new_deprecated("my_macro")
		.old("my_macro()")
		.new("my_macro::new()")
		.help_link("https:://example.com")
		.span(proc_macro2::Span::call_site())
		.build_or_panic();
	let got_tokens = quote!(#warning);

	let want_tokens = quote!(
		/// This function should not be called and only exists to emit a compiler warning.
		///
		/// It is a No-OP in any case.
		#[allow(dead_code)]
		#[allow(non_camel_case_types)]
		#[allow(non_snake_case)]
		fn my_macro() {
			#[deprecated(
				note = "\n\t\tIt is deprecated to my_macro().\n\t\tPlease instead my_macro::new().\n\n\t\tFor more info see:\n\t\t\t<https:://example.com>"
			)]
			#[allow(non_upper_case_globals)]
			const _w: () = ();
			let _ = _w;
		}
	);

	assert_eq!(got_tokens.to_string(), want_tokens.to_string());
}

/// Check the functions that accepting `Into<String>` work as expected.
#[test]
fn type_inferring_into_string_works() {
	macro_rules! test_into_string_inference {
		($($warning:tt)+) => {
			let _ = $($warning)+ ("");
			let _ = $($warning)+ (String::new());
			let _ = $($warning)+ (&String::new());

			#[allow(clippy::from_over_into)]
			{
				struct Custom;
				impl Into<String> for Custom {
					fn into(self) -> String {
						String::new()
					}
				}
				let _ = $($warning)+ (Custom);
			}
		}
	}

	test_into_string_inference!(DeprecatedWarningBuilder::from_title);

	test_into_string_inference!(Warning::new_deprecated);
	test_into_string_inference!(Warning::new_deprecated("").old);
	test_into_string_inference!(Warning::new_deprecated("").new);
	test_into_string_inference!(Warning::new_deprecated("").help_link);
}

#[test]
#[cfg(feature = "derive_debug")]
fn warning_debug_works() {
	let warning = Warning::new_deprecated("my_macro")
		.old("my_macro()")
		.new("my_macro::new()")
		.help_link("https:://example.com")
		.span(proc_macro2::Span::call_site())
		.build_or_panic();
	let _ = format!("{:?}", warning);
}

#[test]
#[cfg(feature = "derive_debug")]
fn formatted_warning_debug_works() {
	let warning =
		FormattedWarning::new_deprecated("my_macro", "my_macro()", proc_macro2::Span::call_site());
	let _ = format!("{:?}", warning);
}

#[test]
#[cfg(feature = "derive_debug")]
fn deprecated_warning_builder_debug_works() {
	let builder = DeprecatedWarningBuilder::from_title("my_macro");
	let _ = format!("{:?}", builder);
}
