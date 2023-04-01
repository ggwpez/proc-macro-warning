<h2><center>Proc Macro Warning</center></h2>
<center>As the name says… emits warnings inside a proc macro.</center>
<br>
Rust does not have native functions to produce warnings from inside proc macros. This crate provides "deprecated" warnings for your proc macro use-cases.
<br>

## Example

Building a warning is easy with the builder pattern.

```rust
use proc_macro_warning::Warning;
let warning = Warning::new_deprecated("my_macro")
	.old("my_macro()")
	.alternative("my_macro::new()")
	.help_link("https:://example.com")
	.span(proc_macro2::Span::call_site())
	.build();

// Use the warning in a proc macro
let tokens = quote::quote!(#warning);
```

## Used In 

Substrate (not yet, but hopefully soon 😉) uses this to emit warnings for its FRAME eDSL on deprecated behaviour.

For example not putting a `call_index` on your functions produces:
```pre
warning: use of deprecated unit struct `pallet::warnings::ImplicitCallIndex::_w`: 
                 It is deprecated to use implicit call indices.
                 Please instead ensure that all calls have the `pallet::call_index` attribute or that the `dev-mode` of the pallet is enabled.
         
                 For more info see:
                     <https://github.com/paritytech/substrate/pull/12891>
                     <https://github.com/paritytech/substrate/pull/11381>
    --> frame/nomination-pools/src/lib.rs:2621:10
     |
2621 |         pub fn claim_commission(origin: OriginFor<T>, pool_id: PoolId) -> DispatchResult {
     |                ^^^^^^^^^^^^^^^^
     |
```

Or using a hard-coded weight:
```pre
warning: use of deprecated unit struct `pallet::warnings::ConstantWeight::_w`: 
                 It is deprecated to use hard-coded constant as call weight.
                 Please instead benchmark all calls or put the pallet into `dev` mode.
         
                 For more info see:
                     <TODO>
    --> frame/nomination-pools/src/lib.rs:2620:20
     |
2620 |         #[pallet::weight(0)]
     |                          
```
