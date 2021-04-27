

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn prusti_polyfill_init(_item: TokenStream) -> TokenStream {
    return "#[cfg(feature = \"verified\")]\nextern crate proc_macro;\n#[cfg(feature = \"verified\")]\nuse proc_macro::TokenStream;".parse().unwrap();
}

#[proc_macro_attribute]
pub fn trusted(_attr: TokenStream, item: TokenStream) -> TokenStream {
	println!("trusting!");
    return item;
}





