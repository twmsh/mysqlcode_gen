use sqlite_model::sqlite_util::{self,MySqxErr};

use sqlx::{query, FromRow, Sqlite, Pool, Row};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Write;
use tokio_stream::StreamExt;

use clap::{Arg, Command};

#[derive(FromRow, Debug, Clone)]
pub struct ColumnDef {
    #[sqlx(rename = "cid")]
    _cid: i32,

    name: String,

    #[sqlx(rename = "type")]
    ty: String,

    #[sqlx(rename = "notnull")]
    not_null: i32,

    #[sqlx(rename = "dflt_value")]
    _default_value: Option<String>,

    pk: i32,
}

//-------------------------------------
#[derive(Debug)]
pub struct EntityAttr {
    pub attr_name: String,
    pub alias: Option<String>,
    pub pk: bool,
    pub ty: String,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct Entity {
    pub table_name: String,
    pub entity_name: String,
    pub comment: Option<String>,

    pub attrs: Vec<EntityAttr>,
}

impl Display for EntityAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref comment) = self.comment {
            let _ = writeln!(f, r#"    /* {} */"#, comment);
        }

        if self.pk {
            let _ = writeln!(f, "    #[pk]");
        }
        if let Some(ref alias) = self.alias {
            let _ = writeln!(f, r#"    #[column = "{}"]"#, alias);
        }
        write!(f, "    pub {}: {},", self.attr_name, self.ty)
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref comment) = self.comment {
            let _ = writeln!(f, r#"/* {} */"#, comment);
        }

        let _ = writeln!(
            f,
            r#"#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]"#
        );
        let _ = writeln!(f, r#"#[table = "{}"]"#, self.table_name);
        let _ = writeln!(f, r#"pub struct {} {{"#, self.entity_name);

        let mut list = Vec::new();

        for attr in self.attrs.iter() {
            list.push(format!("{}\n", attr));
        }
        let list_str = list.join("\n");
        let _ = write!(f, r#"{}"#, list_str);
        write!(f, r#"}}"#)
    }
}

//-------------------------------------
pub fn trim_colum_type(col_type: &str) -> String {
    // 小写，并去掉 (及后面
    let s = col_type.to_lowercase();
    if let Some(p) = s.find("(") {
        s[..p].to_string()
    } else {
        s
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/**
根据表名，生成结构体名称
去掉 "-" "_" 各节首字母大写，然后连在一起
 */
fn build_entity_name(table_name: &str) -> String {
    let table_name = table_name.to_lowercase().replace('-', "_");
    let s: Vec<String> = table_name
        .split('_')
        .map(|x| uppercase_first_letter(&x.to_lowercase()))
        .collect();
    s.concat()
}

fn build_attr_name(column_name: &str) -> String {
    column_name.to_lowercase()
}

fn build_type(column_type: &str, is_null: bool) -> Result<String, sqlx::Error> {
    let column_ty = trim_colum_type(column_type).to_lowercase();

    let ty = match column_ty.as_str() {
        "integer" | "bigint" | "unsigned big int" => "i64",
        "int" | "tinyint" | "smallint" | "mediumint" | "int2" | "int8" => "i32",
        "float" | "real" | "double" | "double precision" => "f64",
        "varchar" | "char" | "varying character" | "nchar" | "native character" | "nvarchar"
        | "text" | "clob" => "String",

        "date" | "datetime" | "timestamp" | "time" => "DateTime<Local>",
        "blob" => "Vec<u8>",
        _ => {
            return Err(MySqxErr(format!("type: {} can't map", column_type)).into());
        }
    };
    if is_null {
        Ok(format!("Option<{}>", ty))
    } else {
        Ok(ty.to_string())
    }
}

fn build_entity_from_columns(
    table_name: String,
    table_comment: String,
    columns: Vec<ColumnDef>,
) -> Result<Entity, sqlx::Error> {
    let entity_name = build_entity_name(table_name.as_str());

    let mut attrs = Vec::new();
    for column in columns.iter() {
        let attr_name = build_attr_name(column.name.as_str());
        let alias = if attr_name.eq(column.name.as_str()) {
            None
        } else {
            Some(column.name.clone())
        };
        let pk = column.pk == 1;
        let mut is_null = column.not_null == 0;
        if pk {
            is_null = false;
        }

        let ty = build_type(column.ty.as_str(), is_null)?;
        let comment = None;
        let attr = EntityAttr {
            attr_name,
            alias,
            pk,
            ty,
            comment,
        };
        attrs.push(attr);
    }

    let comment = if table_comment.is_empty() {
        None
    } else {
        Some(table_comment)
    };
    Ok(Entity {
        table_name,
        entity_name,
        comment,
        attrs,
    })
}

async fn get_columns(pool: &Pool<Sqlite>, table_name: &str) -> sqlx::Result<Vec<ColumnDef>> {
    let sql = format!("pragma table_info('{}')",table_name);

    let mut rows = sqlx::query_as::<_, ColumnDef>(&sql)
        // .bind(table_name)
        .fetch(pool);
    let mut list = Vec::new();

    while let Some(row) = rows.try_next().await? {
        list.push(row);
    }
    Ok(list)
}

//
async fn get_table_names(pool: &Pool<Sqlite>) -> sqlx::Result<Vec<String>> {
    let sql = "select tbl_name from sqlite_master  WHERE type = ? and tbl_name != ? ";

    let mut rows = query(sql).bind("table")
        .bind("sqlite_sequence").fetch(pool);

    let mut list = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let table_name = row.try_get("tbl_name")?;

        list.push(table_name);
    }

    Ok(list)
}

fn render_import() -> String {
    r#"use chrono::{DateTime, Local};
use sqlite_codegen::SqliteEntity;
use serde::{Deserialize, Serialize};
use sqlx::Arguments;"#
        .to_string()
}

async fn write_to_file(entities: &Vec<Entity>, path: &str) -> std::io::Result<()> {
    let mut f = fs::File::create(path)?;

    let header = render_import();
    f.write(format!("{}\n\n", header).as_bytes())?;

    for entity in entities.iter() {
        f.write(format!("{}\n", entity).as_bytes())?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let matches = Command::new("sqlite_gen")
        .version("1.0")
        .author("tom tong")
        .about("generate rust entity code from sqlite db")
        .arg(
            Arg::new("db_url")
                .short('u')
                .long("db_url")
                .required(true)
                .takes_value(true)
                .help("sqlite url"),
        )
        .arg(
            Arg::new("file_path")
                .short('f')
                .long("file_path")
                .required(true)
                .takes_value(true)
                .help("rust file path"),
        )
        .get_matches();

    let db_url = matches.value_of("db_url").unwrap();

    let file_path = matches.value_of("file_path").unwrap();

    // let db_url = "sqlite:todos.db";
    let pool = sqlite_util::init_sqlite_pool(db_url,  4, 1).await?;

    // let db_name = "cf_2.6";
    let tables = get_table_names(&pool).await?;
    println!("Find {} tables", tables.len());

    let mut entity_list = Vec::new();

    for table in tables.iter() {
        let table_name = table.clone();
        let table_comment = "".to_string();

        let column_list = get_columns(&pool, table_name.as_str()).await?;

        let entity = build_entity_from_columns(table_name, table_comment, column_list)?;

        entity_list.push(entity);
    }

    let _ = write_to_file(&entity_list, file_path).await?;
    println!("Write ok, {}", file_path);

    Ok(())
}
