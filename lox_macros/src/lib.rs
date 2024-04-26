mod operations;
mod utils;

use operations::derive_operations;
use proc_macro::TokenStream;

#[proc_macro_derive(OpCodec)]
pub fn operations(input: TokenStream) -> TokenStream {
    derive_operations(input)
}
