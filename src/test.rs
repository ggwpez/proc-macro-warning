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
		.build();
	let got = quote!(#warning);

	let want = quote!(
		#[doc = r" This function should not be called and and only exists to emit a compiler warning."]
		#[doc = r""]
		#[doc = r" It is a No-OP if you want try it anyway ;)"]
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

	assert_eq!(got.to_string(), want.to_string());
}
