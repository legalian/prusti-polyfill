

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn prusti_polyfill_init(_item: TokenStream) -> TokenStream {
	#[cfg(feature = "verified")]
    return "extern crate proc_macro;use proc_macro::TokenStream;".parse().unwrap();
    #[cfg(not(feature = "verified"))]
    return "".parse().unwrap();
}

#[cfg(not(feature = "verified"))]
#[proc_macro_attribute]
pub fn trusted(_attr: TokenStream, item: TokenStream) -> TokenStream {
    return item;
}





