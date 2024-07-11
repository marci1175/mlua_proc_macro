use quote::quote;
use syn::{spanned::Spanned, Fields, ItemStruct};

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
        //If BOTH are true then that means that there is invalid use of the attributes, and we should create a compiler error

        //This is set to true by the closure if we want to skip this entry
        let mut should_skip = false;
        //This is set to true by the closure if we want to save this entry
        let mut should_save = false;

        //If there was the skip attr present we continue, so that we dont generate code for this field
        for attr in &field.attrs {
            //We can manually skip an entry or if there is a serde skip ettribute that means that it cant be turned into a lua table entry anyway
            if attr.path().is_ident("table") || attr.path().is_ident("serde") {
                //Check if there is a skip attribute in the table or serde attribute
                match attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("save") {
                        //If the user wants to save then we let them xd
                        should_save = true;
                        if should_save && should_skip && !attr.path().is_ident("serde") {
                            return Err(syn::Error::new(attr.span(), "You can only save or skip a field."));
                        }
                    }
                    if meta.path.is_ident("skip") {
                        should_skip = true;
                        if should_save && should_skip && !attr.path().is_ident("serde") {
                            return Err(syn::Error::new(attr.span(), "You can only save or skip a field."));
                        }
                    }

                    Ok(())
                })
                {
                    Ok(_) => {},
                    Err(_) => {return quote!(compile_error!("You can only save or skip a field.");).into()}
                };

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

        //Check if the field supports serde
        quote! {
            match serde_json::to_string(&self.#ident) {
                Ok(_) => {TokenStream::new()},
                Err(err) => {syn::Error::into_compiler_error},
            }
        };

        //Statement, this is the code representation of the automaticly added line
        let statement = quote! {
            table.set(#string, serde_json::to_string(&self.#ident).unwrap()).unwrap();
        };

        //Back up all the statements
        statements.push(statement);
    }

    let name = &input.ident;
    let name_as_string = name.to_string().to_lowercase();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    //Create function
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn set_table_from_struct(&self, lua: &mlua::Lua) {
                let table = lua.create_table().unwrap();
                #(#statements)*
                lua.globals().set(#name_as_string, table).unwrap();
            }
        }
    }
    .into()
}
