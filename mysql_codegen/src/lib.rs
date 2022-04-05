use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Ident, Lit, Meta,
    MetaNameValue, Path, Result, Type, TypePath,
};

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
fn find_table_name_from_deriveinput(st: &DeriveInput) -> syn::Result<Option<String>> {
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

fn find_pk_filed(st: &DeriveInput) -> syn::Result<Option<&Field>> {
    let fields = match st.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => {
            return Ok(None);
        }
    };

    for field in fields.iter() {
        for attr in field.attrs.iter() {
            if attr.path.is_ident("pk") {
                return Ok(Some(field));
            }
        }
    }

    Ok(None)
}

fn get_column_name(field: &Field) -> syn::Result<Option<String>> {
    // 查找有没有column属性，没有的，用结构体字段名

    for attr in field.attrs.iter() {
        if attr.path.is_ident("column") {
            if let Meta::NameValue(MetaNameValue {
                lit: Lit::Str(list_str),
                ..
            }) = attr.parse_meta()?
            {
                return Ok(Some(list_str.value()));
            }
        }
    }

    match field.ident {
        None => Ok(None),
        Some(ref v) => Ok(Some(v.to_string())),
    }
}

fn find_datetime_fields(st: &syn::DeriveInput) -> syn::Result<Vec<&Ident>> {
    let fields = match st.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => {
            return Ok(vec![]);
        }
    };

    let mut field_list = Vec::new();

    for field in fields.iter() {
        if let Type::Path(TypePath { ref path, .. }) = field.ty {
            if let Some(seg) = path.segments.first() {
                if seg.ident == "DateTime" {
                    if let Some(ref ident) = field.ident {
                        field_list.push(ident);
                    }

                }
            }
        }
    }

    eprintln!("===> {}",field_list.len());
    eprintln!("===> {:#?}",field_list);

    Ok(field_list)
}

// 生成删除函数
// 获取表名，pk字段名词，类型
//     pub async fn delete_by_id(pool: &Pool<MySql>, id: i64) -> Result<u64, sqlx::Error> {
//         let sql = "delete from be_user where id = ?";
//         let rst = sqlx::query(sql).bind(id).execute(pool).await?;
//         Ok(rst.rows_affected())
//     }
fn generate_delete_function(st: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let table = match find_table_name_from_deriveinput(st)? {
        None => {
            return Err(syn::Error::new_spanned(st, "can't find table attr"));
        }
        Some(v) => v,
    };

    let pk_field = match find_pk_filed(st)? {
        None => {
            return Err(syn::Error::new_spanned(st, "can't find pk attr"));
        }
        Some(v) => v,
    };

    let pk_ident = pk_field.ident.clone().unwrap();
    let ty = &pk_field.ty;

    let column = match get_column_name(pk_field)? {
        None => pk_ident.to_string(),
        Some(v) => v,
    };

    let sql_str = format!("delete from {} where {} = ?", table, column);

    let sql_lit = syn::LitStr::new(sql_str.as_str(), st.span());

    let piece = quote::quote! {
         pub async fn delete_by_id(pool: &sqlx::Pool<sqlx::MySql>, #pk_ident: #ty) -> std::result::Result<u64, sqlx::Error> {
            let sql = #sql_lit;
            let rst = sqlx::query(sql).bind(#pk_ident).execute(pool).await?;
            Ok(rst.rows_affected())
         }
    };
    Ok(piece)
}

// pub async fn get_by_id(
//         pool: &Pool<MySql>,
//         tz: &FixedOffset,
//         id: i64,
//     ) -> Result<Option<Self>, sqlx::Error> {
//         let sql = "select * from be_user where id = ?";
//         let mut rst = sqlx::query_as::<_, BeUser>(sql)
//             .bind(id)
//             .fetch_optional(pool)
//             .await?;
//
//         if let Some(ref mut v) = rst {
//             mysql_util::fix_read_dt_option(&mut v.last_login, tz);
//             mysql_util::fix_read_dt_option(&mut v.token_expire, tz);
//             mysql_util::fix_read_dt(&mut v.gmt_create, tz);
//             mysql_util::fix_read_dt(&mut v.gmt_modified, tz);
//         }
//
//         Ok(rst)
//     }

fn generate_select_by_id(st: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let table = match find_table_name_from_deriveinput(st)? {
        None => {
            return Err(syn::Error::new_spanned(st, "can't find table attr"));
        }
        Some(v) => v,
    };

    let pk_field = match find_pk_filed(st)? {
        None => {
            return Err(syn::Error::new_spanned(st, "can't find pk attr"));
        }
        Some(v) => v,
    };
    let ident = &st.ident;
    let pk_ident = pk_field.ident.clone().unwrap();
    let ty = &pk_field.ty;

    let column = match get_column_name(pk_field)? {
        None => pk_ident.to_string(),
        Some(v) => v,
    };

    let sql_str = format!("select * from {} where {} = ?", table, column);
    let sql_lit = syn::LitStr::new(sql_str.as_str(), st.span());

    let datetime_fields = find_datetime_fields(st)?;

    let date_piece = if datetime_fields.is_empty() {
        quote::quote! {}
    }else{
        quote::quote! {
            if let Some(ref mut v) = rst {
                #(mysql_util::fix_read_dt_option(&mut v.#datetime_fields, tz));*
            }
        }
    };




    let piece = quote::quote! {
        pub async fn get_by_id(
        pool: &sqlx::Pool<sqlx::MySql>,
        tz: &chrono::FixedOffset,
        #pk_ident: #ty,
    ) -> std::result::Result<Option<Self>, sqlx::Error>
        {
            let mut rst = sqlx::query_as::<_, #ident>(sql)
                .bind(#pk_ident)
                .fetch_optional(pool)
                .await?;

            #date_piece
            Ok(rst)
        }

    };
    Ok(piece)
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
    // println!("{:#?}", ast);

    // let table = get_table_name(&ast);
    // eprintln!("table: {:#?}", table);
    // travel_it(&ast);

    let piece_delete_function = generate_delete_function(&ast).unwrap();
    let piece_select_by_id = generate_select_by_id(&ast).unwrap();
    let piece = quote::quote! {
        impl BeUser {
            #piece_delete_function
            #piece_select_by_id

        }
    };

    piece.into()

    // TokenStream::new()
}
