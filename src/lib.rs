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

    'field_gen: for field in &fields.named {
        //If there was the skip attr present we continue, so that we dont generate code for this field
        for attr in &field.attrs {
            //We can manually skip an entry or if there is a serde skip ettribute that means that it cant be turned into a lua table entry anyway
            if attr.path().is_ident("table") || attr.path().is_ident("serde") {

                //This is set to true by the closure if we want to skip this entry
                let mut should_skip = false;

                //Check if there is a skip attribute in the table or serde attribute
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        should_skip = true;
                    }

                    Ok(())
                })
                .unwrap();

                //If we should skip this entry
                if should_skip {
                    continue 'field_gen;
                }
            }
        }

        //Name of the entry
        let ident = field.ident.as_ref().unwrap();
        
        //Name of the entry as string
        let string = ident.to_string();

        //Statement, this is the code representation of the automaticly added line
        let statement = quote! {
            table.set(#string, serde_json::to_string(&self.#ident).unwrap());
        };

        //Back up all the statements
        statements.push(statement);
    }

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    //Create function
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
