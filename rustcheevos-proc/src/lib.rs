use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Chains together multiple requirements.
///
/// # Examples
/// ```
/// use rustcheevos::{prelude::*, bits8, chain, delta};
///
/// chain!(
///     delta!(bits8!(0x1234)).lt(10),
///     bits8!(0x1234).ge(10),
/// );
/// ```
#[proc_macro]
pub fn chain(input: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(input as ChainInput);

    let (chain, head) = match exprs.items.as_slice() {
        [] => panic!("chain! requires at least one argument"),
        [head] => (vec![], head),
        [pending @ .., head] => (pending.to_vec(), head),
    };

    quote! {
        {
            let mut group = rustcheevos::types::chain::Chain::new();
            #(group.extend(#chain);)*
            #head.chain(group)
        }
    }
    .into()
}

struct ChainInput {
    items: Vec<syn::Expr>,
}

impl syn::parse::Parse for ChainInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
            if input.is_empty() {
                break;
            }
            input.parse::<syn::Token![,]>()?;
        }
        Ok(ChainInput { items })
    }
}
