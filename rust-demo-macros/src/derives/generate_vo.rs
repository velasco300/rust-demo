use quote::quote;
use syn::spanned::Spanned;

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

pub fn get_fields_from_derive_input(ts: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ts.data
    {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(
        ts,
        "Must define on a Struct, not Enum".to_string(),
    ))
}

pub fn generate_vo_struct_fields_def(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let token_stream = quote! {
        #(pub #idents: std::option::Option<#types>),*
        pub page_num: std::option::Option<i32>,
        pub page_size: std::option::Option<i32>,
    };

    Ok(token_stream)
}

pub fn generate_active_model_fields_def(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_tokenstream = proc_macro2::TokenStream::new();
    fields.iter().for_each(|f| {
        let ident = &f.ident;
        let ts = quote! {
            #ident: if let Some(x) = self.#ident {
                sea_orm::Set(x)
            } else {
                sea_orm::NotSet
            },
        };
        final_tokenstream.extend(ts);
    });

    Ok(final_tokenstream)
}

pub fn expand_derive_generate_vo(ts: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = ts.ident.to_string();
    let vo_name_literal = format!("{}VO", struct_name_literal);
    let vo_name_ident = syn::Ident::new(&vo_name_literal, ts.span());

    let fields = get_fields_from_derive_input(ts)?;
    let vo_struct_fields_def = generate_vo_struct_fields_def(fields)?;
    let active_model_fields_def = generate_active_model_fields_def(fields)?;

    let ret = quote! {
        #[derive(Debug, Default)]
        pub struct #vo_name_ident {
            #vo_struct_fields_def
        }
        impl #vo_name_ident {
            pub fn into_active_model(self) -> ActiveModel {
                ActiveModel {
                    #active_model_fields_def
                }
            }
        }
    };

    return Ok(ret);
}
