

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn prusti_polyfill_init(_item: TokenStream) -> TokenStream {
return "
	#[cfg(feature = \"verified\")]
	extern crate prusti_contracts;
	#[cfg(feature = \"verified\")]
	use prusti_contracts::trusted;
".parse().unwrap();
}

#[proc_macro_attribute]
pub fn trusted(_attr: TokenStream, item: TokenStream) -> TokenStream {
	println!("trusting!");
	return item;
}





