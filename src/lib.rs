use proc_macro::{TokenStream, TokenTree};
use proc_macro::TokenTree::Group;
#[proc_macro_derive(ToTable)]
pub fn convert_to_table(input: TokenStream) -> TokenStream {
    
    for token in input {
        match token {
            //The struct fields
            Group(group) => {
                let mut last_token: Option<TokenTree> = None;
                let mut field_pairs: Vec<(TokenTree, TokenTree)> = Vec::new();

                for inner_item in group.stream().into_iter().step_by(2) {
                    if let Some(token) = &last_token {
                        field_pairs.push((token.clone(), inner_item));

                        last_token = None;
                    }
                    else {
                        last_token = Some(inner_item);
                    }
                }

                dbg!(field_pairs);
            }
            _ => {},
        };
    }

    TokenStream::new()
}