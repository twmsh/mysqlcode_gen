use mysql_model::mysql_util;
use mysql_model::mysql_util::MySqxErr;
use sqlx::{query, query_as, FromRow, MySql, Pool, Row};
use tokio_stream::StreamExt;

#[derive(FromRow, Debug, Clone)]
pub struct ColumnDef {
    #[sqlx(rename = "COLUMN_NAME")]
    column_name: String,

    #[sqlx(rename = "DATA_TYPE")]
    data_type: String,

    #[sqlx(rename = "IS_NULLABLE")]
    is_nullable: String,

    #[sqlx(rename = "COLUMN_KEY")]
    column_key: String,

    #[sqlx(rename = "COLUMN_COMMENT")]
    column_comment: String,
}

async fn get_columns(
    pool: &Pool<MySql>,
    db_name: &str,
    table_name: &str,
) -> sqlx::Result<Vec<ColumnDef>> {
    let sql = "select COLUMN_NAME,DATA_TYPE,IS_NULLABLE,COLUMN_KEY,COLUMN_COMMENT \
    FROM information_schema.COLUMNS WHERE table_schema = ? and table_name = ?  \
    order by ORDINAL_POSITION asc";

    let mut rows = sqlx::query_as::<_, ColumnDef>(sql)
        .bind(db_name)
        .bind(table_name)
        .fetch(pool);
    let mut list = Vec::new();

    while let Some(row) = rows.try_next().await? {
        list.push(row);
    }
    Ok(list)
}

//
async fn get_table_names(pool: &Pool<MySql>, db_name: &str) -> sqlx::Result<Vec<(String, String)>> {
    let sql = "select table_name,table_comment from information_schema.tables WHERE table_schema = ? and table_type = ? ";

    let mut rows = query(sql).bind(db_name).bind("BASE TABLE").fetch(pool);

    let mut list = Vec::new();
    while let Some(row) = rows.try_next().await? {
        let table_name = row.try_get("table_name")?;
        let table_comment = row.try_get("table_comment")?;
        list.push((table_name, table_comment));
    }

    Ok(list)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("aaa");
    let tz = "+08:00";
    let db_url = "mysql://root:cf123456@192.168.1.26:3306";
    let pool = mysql_util::init_pool(db_url, tz, 4, 1).await?;
    println!("bbb");
    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let db_name = "cf_2.6";
    let tables = get_table_names(&pool, db_name).await?;
    println!("{:#?}", tables);

    println!("++++++++++++++++++++++++");

    for table in tables.iter() {
        let table_name = table.0.clone();
        println!("-------------- {} ---------------", table_name);
        let column_list = get_columns(&pool, db_name, table_name.as_str()).await?;
        println!("{:?}", column_list);
    }

    Ok(())
}
