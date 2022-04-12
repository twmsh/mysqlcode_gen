#![allow(dead_code, unused_imports)]
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};

use sqlx::{Arguments, Pool, Row, Sqlite};
use tokio_stream::StreamExt;

use sqlite_model::sqlite_util;
use sqlite_model::sqlite_util::MySqxErr;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct BeUser {
    pub id: i32,
    pub name: Option<String>,
    pub login_name: String,
    pub password: String,
    pub salt: String,

    pub token: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub service_flag: Option<i16>,
    pub ref_count: Option<i16>,

    pub last_login: Option<DateTime<Local>>,
    pub token_expire: Option<DateTime<Local>>,
    pub memo: Option<String>,
    pub gmt_create: DateTime<Local>,
    pub gmt_modified: DateTime<Local>,
}

impl BeUser {
    pub async fn get_by_id(
        pool: &Pool<Sqlite>,
        id: i64,
    ) -> Result<Option<Self>, sqlx::Error> {
        let sql = "select * from be_user where id = ?";
        let  rst = sqlx::query_as::<_, BeUser>(sql)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(rst)
    }

    pub async fn delete_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<u64, sqlx::Error> {
        let sql = "delete from be_user where id = ?";
        let rst = sqlx::query(sql).bind(id).execute(pool).await?;
        Ok(rst.rows_affected())
    }

    pub async fn insert(&self, pool: &Pool<Sqlite>) -> Result<u64, sqlx::Error> {
        let sql = "insert into be_user(name,login_name,password,salt, token,phone,email,service_flag,ref_count,last_login,token_expire,memo,gmt_create,gmt_modified) values(?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
        let mut args = sqlx::sqlite::SqliteArguments::default();

        args.add(self.name.clone());
        args.add(self.login_name.clone());
        args.add(self.password.clone());
        args.add(self.salt.clone());

        args.add(self.token.clone());
        args.add(self.phone.clone());
        args.add(self.email.clone());
        args.add(self.service_flag.clone());
        args.add(self.ref_count.clone());

        args.add(self.last_login.clone());
        args.add(self.token_expire.clone());
        args.add(self.memo.clone());
        args.add(self.gmt_create.clone());
        args.add(self.gmt_modified.clone());

        let rst = sqlx::query_with(sql, args).execute(pool).await?;
        Ok(rst.last_insert_rowid() as u64)
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let tz = "+08:00";
    // let offset = match sqlite_util::parse_timezone(tz) {
    //     Ok(v) => v,
    //     Err(e) => {
    //         return Err(MySqxErr(e).into());
    //     }
    // };


    let db_url = r#"sqlite:C:\Users\tom\develop\RustProjects\mysql_codegen\doc\a.db"#;
    let pool = sqlite_util::init_sqlite_pool(db_url,4,2).await?;

    let obj = BeUser::get_by_id(&pool,1).await?;
    println!("{:?}",obj);

    Ok(())
}