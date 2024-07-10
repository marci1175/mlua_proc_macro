use quote::quote;
use syn::{Fields, ItemStruct};

#[proc_macro_derive(ToTable, attributes(table))]
pub fn convert_to_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    convert_to_table_impl(syn::parse_macro_input!(input))
}

fn convert_to_table_impl(input: ItemStruct) -> proc_macro::TokenStream {
    let Fields::Named(fields) = &input.fields else {
        return quote!(compile_error!("expected struct with named fields");).into();
    };

    let mut statements = vec![];

    for field in &fields.named {
        //If there was the skip attr present we continue, so that we dont generate code for this field
        for attr in &field.attrs {
            if attr.path().is_ident("table") {
                let mut should_skip = false;

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        should_skip = true;
                    }

                    Ok(())
                })
                .unwrap();

                if should_skip {
                    continue;
                }
            }
        }

        let ident = field.ident.as_ref().unwrap();
        let string = ident.to_string();
        let statement = quote! {
            table.set(#string, serde_json::to_string(&self.#ident).unwrap());
        };

        statements.push(statement);
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            fn set_table_from_struct(&self, lua: &mlua::Lua) {
                let table = lua.create_table().unwrap();
                #(#statements)*
                lua.globals().set("vars", table).unwrap();
            }
        }
    }
    .into()
}
