

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn prusti_polyfill_init(_item: TokenStream) -> TokenStream {
return "
	#[cfg(feature = \"verified\")]
	extern crate prusti_contracts;
	#[cfg(feature = \"verified\")]
	use prusti_contracts::*;
	#[cfg(not(feature = \"verified\"))]
	use prusti_polyfill::*;
".parse().unwrap();
}

#[proc_macro_attribute]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn ensures(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn after_expiry(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn after_expiry_if(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn pure(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn trusted(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro]
pub fn body_invariant(_item: TokenStream) -> TokenStream {return "".parse().unwrap();}
macro_rules! closure {
    ($($b:tt)*,$a:expr)=>{$a}
}
#[proc_macro_attribute]
pub fn refine_trait_spec(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn extern_spec(_attr: TokenStream, _item: TokenStream) -> TokenStream {return "".parse().unwrap();}
#[proc_macro_attribute]
pub fn predicate(_attr: TokenStream, _item: TokenStream) -> TokenStream {return "".parse().unwrap();}

#[proc_macro_attribute]
pub fn refine_requires(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute]
pub fn refine_ensures(_attr: TokenStream, item: TokenStream) -> TokenStream {item}

