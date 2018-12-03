extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use self::proc_macro::TokenStream;
use self::syn::{parse, parse::Parser, punctuated::Punctuated, Expr, ItemFn};

#[proc_macro_attribute]
pub fn aoc(meta: TokenStream, item: TokenStream) -> TokenStream {
    let meta_info = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse(meta)
        .expect("Failed to parse year, day and part!")
        .into_iter()
        .collect::<Vec<_>>();

    let (year, day, part) = match &*meta_info {
        [year, day, part] => (year, day, part),
        _ => panic!(""),
    };

    let fun = parse::<ItemFn>(item)
        .ok()
        .expect("aoc can only be applied to functions");
    
    let name = fun.ident.clone();
    
    let ret = quote!(
        fn #name() {
            #fun
            ::aoc::cli::run(#year.to_string(), #day, #part, #name);
        }
    );

    ret.into()
}
