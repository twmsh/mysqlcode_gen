use std::time::Duration;
use chrono::{DateTime, Local};
use sqlx::{Executor, MySql,Pool};
use sqlx::mysql::MySqlPoolOptions;

use serde::{Serialize,Deserialize};

#[derive(sqlx::FromRow)]
#[derive(Serialize,Deserialize,Debug,Clone)]
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

async fn fetch_one(pool: &Pool<MySql>,id: i32) -> Result<Option<BeUser>,sqlx::Error> {
    let sql = "select * from be_user where id = ?";
    let entity = sqlx::query_as::<_,BeUser>(sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(entity)
}

#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    println!("aaa");
    // let db_url = "mysql://cf_user:cf123456@localhost:3306/cf_rs";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .idle_timeout(Duration::from_secs(60 * 30))
        .after_connect(|conn| Box::pin(
            async move {
                let sql = format!("set time_zone = '{}'", "+04:00");

                conn.execute(sql.as_str()).await?;
                println!("set end.");
                Ok(())
            }
        ))
        .connect(db_url).await?;

    let entity = fetch_one(&pool,1).await;

    println!("fetch_one: {:?}",entity);


    Ok(())
}