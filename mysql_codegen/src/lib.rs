use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = st.data
    {
        Ok(named)
    } else{
        Err(syn::Error::new_spanned(&st, "can't find Punctuated".to_string()))
    }
}


#[proc_macro_derive(MysqlEntity)]
pub fn mysql_entity(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    println!("ast.data: {:#?}", ast.data);

    println!("ast : {:?}", ast.ident);
    let my_ident = &ast.ident;

    TokenStream::from(quote! {
        impl #my_ident {
            pub async fn get_by_id(pool: &sqlx::Pool<sqlx::MySql>,tz: &chrono::FixedOffset,id: i64)
             -> Result<Option<Self>,sqlx::Error> {

            }
            pub async fn delete_by_id(pool: &sqlx::Pool<sqlx::MySql>,id: i64)
            -> Result<u64, sqlx::Error> {

            }
            pub async fn insert(&self, pool: &sqlx::Pool<sqlx::MySql>, tz: &chrono::FixedOffset)
            -> Result<u64, sqlx::Error>  {

            }

        }



    })

    // TokenStream::new()
}
