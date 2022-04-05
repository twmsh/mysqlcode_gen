use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Lit, Meta, MetaNameValue, Path, Result};

struct Column {
    column_name: String,
    attr_name: String,
}

struct Table {
    table_name: String,
    pk_column: String,
    columns: Vec<Column>,
}

//--------------------------------------

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

// #[derive(MysqlEntity)]
// #[derive(Debug)]
// #[table="cf_source"]
// pub struct CfSource {
//     #[pk]
//     #[column="id"]
//     pub id: i64,
//     pub name: String,
//
//     #[column="create_time"]
//     pub gmt_create: DateTime<Local>,
// }

// 查找表名
fn find_table_name_from_deriveinput(
    st: &DeriveInput,
    ident: String,
) -> syn::Result<Option<String>> {
    for attr in st.attrs.iter() {
        if attr.path.is_ident("table") {
            if let Meta::NameValue(MetaNameValue {
                lit: Lit::Str(list_str),
                ..
            }) = attr.parse_meta()?
            {
                return Ok(Some(list_str.value()));
            }
        }
    }
    Ok(None)
}

fn find_pk_filed(st:&DeriveInput) -> syn::Result<Option<&Field>> {
    let fields = match  st.data {
        Data::Struct(DataStruct{ fields:Fields::Named(FieldsNamed{ref named,..}),..}) => {
                named
        }
       _ => {
           return Ok(None);
       }
    };

    for field in fields.iter() {

    }



    Ok(None)
}


fn find_attribute_from_field(field: &Field, ident: String) -> syn::Result<Option<&Attribute>> {}

// 生成删除函数
// 获取表名，pk字段名词，类型
//     pub async fn delete_by_id(pool: &Pool<MySql>, id: i64) -> Result<u64, sqlx::Error> {
//         let sql = "delete from be_user where id = ?";
//         let rst = sqlx::query(sql).bind(id).execute(pool).await?;
//         Ok(rst.rows_affected())
//     }
fn generate_delete_function(st: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    Ok(TokenStream2::new())
}

fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = st.data
    {
        Ok(named)
    } else {
        Err(syn::Error::new_spanned(
            &st,
            "can't find Punctuated".to_string(),
        ))
    }
}

fn travel_it(st: &syn::DeriveInput) {
    eprintln!("==== struct attr =======");
    for attr in st.attrs.iter() {
        eprintln!("{:#?}", attr);
    }

    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &st.data
    {
        eprintln!("{:#?}", fields.named.len());

        for field in fields.named.iter() {
            eprintln!("---------------------");

            eprintln!("{:#?}", field.attrs);
        }
    }
}

fn get_table_name(st: &syn::DeriveInput) -> syn::Result<String> {
    for attr in st.attrs.iter() {
        // eprintln!("attr: {:#?}",attr);

        // eprintln!("{:#?}",attr.parse_meta());

        if attr.path.is_ident("table") {
            if let Ok(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(list_str),
                ..
            })) = attr.parse_meta()
            {
                // let value:Path = list_str.parse().unwrap();
                // println!("--> list_str: {:?}", list_str);
                return Ok(list_str.value());
            }
        }
    }

    return Err(syn::Error::new_spanned(&st, "can't find table".to_string()));
}

#[proc_macro_derive(MysqlEntity, attributes(table, pk, column))]
pub fn mysql_entity(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    println!("{:#?}", ast);

    // let table = get_table_name(&ast);
    // eprintln!("table: {:#?}", table);
    travel_it(&ast);
    TokenStream::new()
}
