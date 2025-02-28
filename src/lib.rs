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
        //This is set to true if the user has skipped the serde serialization, this means that we would skip this table entry, but if the user manually saves it by ```#[table(save)]``` we save that entry
        let mut should_skip_serde = false;

        //If there was the skip attr present we continue, so that we dont generate code for this field
        for attr in &field.attrs {
            //We can manually skip an entry or if there is a serde skip ettribute that means that it cant be turned into a lua table entry anyway
            if attr.path().is_ident("table") || attr.path().is_ident("serde") {
                //Check if there is a skip attribute in the table or serde attribute
                match attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("save") {
                        //If the user wants to save then we let them xd
                        should_save = true;

                        if should_save && should_skip {
                            return Err(syn::Error::new(
                                attr.span(),
                                "You can only save or skip a field.",
                            ));
                        }
                    }

                    if meta.path.is_ident("skip") {
                        if attr.path().is_ident("serde") {
                            should_skip_serde = true;
                        }
                        //If attr.path().is_ident("table")
                        else {
                            //If the user wants to skip then we set the bool
                            should_skip = true;

                            if should_save && should_skip {
                                return Err(syn::Error::new(
                                    attr.span(),
                                    "You can only save or skip a field.",
                                ));
                            }
                        }
                    }

                    Ok(())
                }) {
                    Ok(_) => {}
                    Err(_) => {
                        return quote!(compile_error!("You can only save or skip a field.");).into()
                    }
                };
            }
        }

        //We should only check for this after we have iterated over the attributes of said field
        //If we should skip this entry
        if should_skip || (should_skip_serde && !should_save) {
            continue 'field_gen;
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
            fields.add_field_method_get(#string, |_, this| { Ok(serde_json::to_string(&this.#ident).unwrap()) });
        };

        //Back up all the statements
        statements.push(statement);
    }

    let name = &input.ident;
    let name_as_string = name.to_string().to_lowercase();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    //Create function
    quote! {
        impl mlua::UserData for #impl_generics #name #ty_generics #where_clause {
            fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
                #(#statements)*
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn set_lua_table_function(self, lua: &Lua) {
                lua.globals().set(#name_as_string, self).unwrap();
            }
        }
    }
    .into()
}
