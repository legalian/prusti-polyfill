

extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use quote::{ToTokens};

struct ClosureWithSpec {
    pub cl: syn::Expr
}
impl Parse for ClosureWithSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        while !input.is_empty() {
            if !input.peek(syn::Ident) {
                return Ok(ClosureWithSpec {
                	cl: input.parse()?
                });
            }
        }
        return Err(syn::Error::new(input.span(), "closure specification without closure"));
    }
}


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
#[proc_macro]
pub fn closure(tokens: TokenStream) -> TokenStream {
	let cl_spec: ClosureWithSpec = match syn::parse(tokens.into()) {
		Ok(data) => data,
		Err(err) => return proc_macro::TokenStream::from(err.to_compile_error())
	};
	return cl_spec.cl.into_token_stream().into();
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

