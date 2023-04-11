/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: (GPL-3.0 or Apache-2.0)
 */

//! Emit warnings from inside proc macros.

use core::ops::Deref;
use proc_macro2::Span;

mod test;

/// Creates a compile-time warning for proc macro use. See [DeprecatedWarningBuilder] for usage.
pub struct Warning {
	pub name: String,
	pub index: Option<usize>,
	pub message: String,
	pub links: Vec<String>,
	pub span: Span,
}

/// Gradually build a "deprecated" `Warning`.
///
/// # Example
/// ```
/// use proc_macro_warning::Warning;
///
/// let warning = Warning::new_deprecated("my_macro")
///     .old("my_macro()")
///     .new("my_macro::new()")
///     .help_link("https:://example.com")
///     .span(proc_macro2::Span::call_site())
///     .build();
///
/// // Use the warning in a proc macro
/// let tokens = quote::quote!(#warning);
///
/// let warnings = vec![warning];
/// // In a proc macro you would expand them inside a module:
/// quote::quote! {
///     mod warnings {
///         #(
///             #warnings
///         )*
///     }
/// };
/// ```
#[derive(Default)]
pub struct DeprecatedWarningBuilder {
	title: String,
	index: Option<usize>,
	old: Option<String>,
	new: Option<String>,
	links: Vec<String>,
	span: Option<Span>,
}

impl DeprecatedWarningBuilder {
	/// Create a new *deprecated* warning builder with the given title.
	///
	/// The title must be unique for each warning.
	#[must_use]
	pub fn from_title(title: &str) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { title: title.into(), ..Default::default() }
	}

	/// Set an optional index in case that a warning appears multiple times.
	///
	/// Must be set if a warning appears multiple times.
	#[must_use]
	pub fn index(self, index: usize) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { index: Some(index), ..self }
	}

	/// The old *deprecated* way of doing something.
	///
	/// Should complete the sentence "It is deprecated to ...".
	#[must_use]
	pub fn old(self, old: &str) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { old: Some(old.into()), ..self }
	}

	/// The *new* way of doing something.
	///
	/// Should complete the sentence "Please instead ...".
	#[must_use]
	pub fn new(self, new: &str) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { new: Some(new.into()), ..self }
	}

	/// A help link for the user to explain the transition and justification.
	#[must_use]
	pub fn help_link(self, link: &str) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { links: vec![link.into()], ..self }
	}

	/// Multiple help links for the user to explain the transition and justification.
	#[must_use]
	pub fn help_links(self, links: &[&str]) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { links: links.iter().map(|s| s.deref().into()).collect(), ..self }
	}

	/// The span of the warning.
	#[must_use]
	pub fn span(self, span: Span) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder { span: Some(span), ..self }
	}

	/// Build the warning.
	pub fn maybe_build(self) -> Result<Warning, String> {
		let span = self.span.unwrap_or_else(Span::call_site);
		let title = self.title;
		let old = self.old.ok_or("Missing old")?;
		let new = self.new.ok_or("Missing new")?;
		let message = format!("It is deprecated to {}.\nPlease instead {}.", old, new);

		Ok(Warning { name: title, index: self.index, message, links: self.links, span })
	}

	/// Unwraps [`Self::maybe_build`] for convenience.
	#[must_use]
	pub fn build(self) -> Warning {
		self.maybe_build().expect("maybe_build failed")
	}
}

impl Warning {
	/// Create a new *raw* warnings.
	pub fn new_raw(
		name: String,
		index: Option<usize>,
		message: String,
		help_links: Vec<String>,
		span: Span,
	) -> Warning {
		Warning { name, index, message, links: help_links, span }
	}

	/// Create a new *deprecated* warning.
	#[must_use]
	pub fn new_deprecated(title: &str) -> DeprecatedWarningBuilder {
		DeprecatedWarningBuilder::from_title(title)
	}

	/// Sanitize the warning message.
	fn final_message(&self) -> String {
		let lines = self.message.trim().lines().map(|line| line.trim_start());
		// Prepend two tabs to each line
		let message = lines.map(|line| format!("\t\t{}", line)).collect::<Vec<_>>().join("\n");

		if !self.links.is_empty() {
			let link = self
				.links
				.iter()
				.map(|l| format!("<{}>", l))
				.collect::<Vec<_>>()
				.join("\n\t\t\t");
			format!("\n{}\n\n\t\tFor more info see:\n\t\t\t{}", message, link)
		} else {
			format!("\n{}", message)
		}
	}

	/// Sanitize the warning name.
	fn final_name(&self) -> syn::Ident {
		let name = match self.index {
			Some(i) => format!("{}_{}", self.name, i),
			None => self.name.clone(),
		};
		syn::Ident::new(&name, self.span)
	}
}

impl quote::ToTokens for Warning {
	fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
		let name = self.final_name();
		let message = self.final_message();

		let q = quote::quote_spanned!(self.span =>
			/// This function should not be called and and only exists to emit a compiler warning.
			///
			/// It is a No-OP if you want try it anyway ;)
			#[allow(dead_code)]
			#[allow(non_camel_case_types)]
			#[allow(non_snake_case)]
			fn #name() {
				#[deprecated(note = #message)]
				#[allow(non_upper_case_globals)]
				const _w: () = ();
				let _ = _w;
			}
		);
		q.to_tokens(stream);
	}
}
