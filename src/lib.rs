

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn prusti_polyfill_init(_item: TokenStream) -> TokenStream {
    return "#[cfg(feature = \"verified\")]\nextern crate prusti_contracts;\n#[cfg(feature = \"verified\")]\nuse prusti_contracts::*;".parse().unwrap();
}

#[proc_macro_attribute]
pub fn trusted(_attr: TokenStream, item: TokenStream) -> TokenStream {
	println!("trusting!");
    return item;
}





