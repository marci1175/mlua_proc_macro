use proc_macro::TokenTree::Group;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(ToTable)]
pub fn convert_to_table(input: TokenStream) -> TokenStream {
    let mut field_pairs: Vec<(TokenTree, TokenTree)> = Vec::new();
    let struct_name: TokenTree = input.clone().into_iter().nth(1).unwrap();

    for token in input {
        match token {
            //The struct fields
            Group(group) => {
                let mut last_token: Option<TokenTree> = None;

                for inner_item in group.stream().into_iter().step_by(2) {
                    if let Some(token) = &last_token {
                        field_pairs.push((token.clone(), inner_item));

                        last_token = None;
                    } else {
                        last_token = Some(inner_item);
                    }
                }
            }
            _ => {}
        };
    }

    let table_fields = field_pairs
        .iter()
        .map(|(name, _)| {
            format!(
                r#"table.set("{}", self.{}).unwrap();"#,
                name.to_string(),
                name.to_string()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"impl {} {{
        fn set_table_from_struct(&self, lua: &mlua::Lua) {{

            let table = lua.create_table().unwrap();

            {}

            lua.globals().set("vars", table).unwrap();
        }}
    }}
    "#,
        struct_name.to_string(),
        table_fields
    )
    .parse()
    .unwrap()
}
