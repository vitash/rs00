// extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro]
pub fn run_fn1(_item: TokenStream) -> TokenStream {
    "local_fn1();".parse().unwrap()
}

// cargo test test_ -p macro1
#[test]
fn test_add() {
    assert_eq!(3, (2))
}
