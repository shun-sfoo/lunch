use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::builder::BuilderContext;

mod builder;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let builderContext = BuilderContext::new(input);
    builderContext.generate().into()

    // println!("{:#?}", input);

    // TokenStream::default()
}
