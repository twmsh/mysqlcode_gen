use chrono::{DateTime, FixedOffset, Local, LocalResult, NaiveDateTime, ParseResult, TimeZone};
use sqlx::error::DatabaseError;
use sqlx::mysql::{MySqlArguments, MySqlPoolOptions};
use sqlx::{Arguments, Executor, MySql, Pool, Row};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

use mysql_model::mysql_util;
use serde::{Deserialize, Serialize};

use tokio_stream::StreamExt;

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

async fn insert_one_with_args(
    pool: &Pool<MySql>,
    db_offset: &FixedOffset,
    login_name: String,
) -> Result<u64, sqlx::Error> {
    let sql = "insert into be_user(login_name,password,salt,service_flag,gmt_create,gmt_modified) values(?,?,?,?,?,?)";
    let mut args = MySqlArguments::default();

    let now = Local::now();

    args.add(login_name.clone());
    args.add(login_name.clone());
    args.add(login_name.clone());
    args.add(Some(100_i16));
    args.add(mysql_util::fix_write_dt(&now, db_offset));
    args.add(mysql_util::fix_write_dt(&now, db_offset));

    let entity = sqlx::query_with(sql, args).execute(pool).await?;

    println!("insert: {:?}", entity);

    Ok(entity.last_insert_id())
}

async fn fetch_count(pool: &Pool<MySql>, flag: i32) -> Result<u64, sqlx::Error> {
    let sql = "select count(*) from be_user where service_flag = ?";
    let row = sqlx::query(sql).bind(flag).fetch_one(pool).await?;

    let count: i64 = row.try_get(0)?;

    Ok(count as u64)
}

async fn fetch_list_unstruct(
    pool: &Pool<MySql>,
    flag: i32,
    db_offset: &FixedOffset,
) -> Result<Vec<(i64, String, DateTime<Local>)>, sqlx::Error> {
    let sql = "select id,login_name,gmt_create from be_user where service_flag = ?";
    let mut rows = sqlx::query(sql).bind(flag).fetch(pool);

    let mut list = Vec::new();

    while let Some(row) = rows.try_next().await? {
        let id = row.try_get("id")?;
        let login_name = row.try_get("login_name")?;
        let mut gmt_create = row.try_get("gmt_create")?;
        mysql_util::fix_read_dt(&mut gmt_create, db_offset);
        list.push((id, login_name, gmt_create));
    }

    Ok(list)
}

async fn fetch_list_page(
    pool: &Pool<MySql>,
    db_offset: &FixedOffset,
    flag: i32,
    start: i64,
    len: i64,

    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
) -> Result<Vec<(i64, String, DateTime<Local>)>, sqlx::Error> {
    let sql = "select id,login_name,gmt_create from be_user where service_flag = ? \
    and gmt_create >= ? and gmt_create < ? order by gmt_create asc limit ?,?";

    let start_time_ex = mysql_util::fix_write_dt(&start_time,db_offset);
    let end_time_ex = mysql_util::fix_write_dt(&end_time,db_offset);


    let mut rows = sqlx::query(sql)
        .bind(flag)
        .bind(start_time_ex)
        .bind(end_time_ex)
        .bind(start)
        .bind(len)
        .fetch(pool);

    let mut list = Vec::new();

    while let Some(row) = rows.try_next().await? {
        let id = row.try_get("id")?;
        let login_name = row.try_get("login_name")?;
        let mut gmt_create = row.try_get("gmt_create")?;
        mysql_util::fix_read_dt(&mut gmt_create, db_offset);
        list.push((id, login_name, gmt_create));
    }

    Ok(list)
}

async fn fetch_one_with_args(
    pool: &Pool<MySql>,
    db_offset: &FixedOffset,
) -> Result<Option<BeUser>, sqlx::Error> {
    let sql = "select * from be_user where id = ? and login_name = ? ";
    let mut args = MySqlArguments::default();

    args.add(1);
    args.add("ccc");
    let entity = sqlx::query_as_with::<_, BeUser, _>(sql, args)
        .fetch_optional(pool)
        .await?;

    println!("before: {:?}", entity);

    match entity {
        Some(mut v) => {
            mysql_util::fix_read_dt(&mut v.gmt_create, db_offset);
            mysql_util::fix_read_dt(&mut v.gmt_modified, db_offset);

            Ok(Some(v))
        }
        None => Ok(None),
    }
}

async fn fetch_one(
    pool: &Pool<MySql>,
    id: i32,
    db_offset: &FixedOffset,
) -> Result<Option<BeUser>, sqlx::Error> {
    let sql = "select * from be_user where id = ?";
    let entity = sqlx::query_as::<_, BeUser>(sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;

    println!("before: {:?}", entity);

    match entity {
        Some(mut v) => {
            mysql_util::fix_read_dt(&mut v.gmt_create, db_offset);
            mysql_util::fix_read_dt(&mut v.gmt_modified, db_offset);

            Ok(Some(v))
        }
        None => Ok(None),
    }
}

#[tokio::main]
pub async fn main() -> Result<(), sqlx::Error> {
    println!("aaa");
    let tz = "+08:00";
    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            println!("error: {}", e);
            return Ok(());
        }
    };

    println!("offset: {}", offset);

    // let db_url = "mysql://cf_user:cf123456@localhost:3306/cf_rs";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .idle_timeout(Duration::from_secs(60 * 30))
        .after_connect(|conn| {
            Box::pin(async move {
                let sql = format!("set time_zone = '{}'", "+04:00");

                conn.execute(sql.as_str()).await?;
                println!("set end.");
                Ok(())
            })
        })
        .connect(db_url)
        .await?;

    // let entity = fetch_one(&pool, 4, &offset).await;
    // println!("fetch_one, after: {:?}", entity);

    // let entity = fetch_one_with_args(&pool,  &offset).await;
    // println!("fetch_one, after: {:?}", entity);

    // let id = insert_one_with_args(&pool,&offset,"ggg".to_string()).await?;
    // println!("insert id: {}",id);

    // let count = fetch_count(&pool,100).await?;
    // println!("count: {}", count);

    // let list = fetch_list_unstruct(&pool, 100, &offset).await?;
    // println!("list: {:?}", list);

    let fmt = "%Y-%m-%d %H:%M:%S.%3f";
    let start_time = parse_local_time_str("2022-03-29 12:55:05.171", fmt)?;
    let end_time = parse_local_time_str("2022-03-29 23:59:05.171", fmt)?;
    println!("start_time: {}", start_time);
    println!("end_time: {}", end_time);

    let list = fetch_list_page(&pool, &offset, 100, 0, 3, start_time, end_time).await?;
    println!("list: {:?}", list);

    Ok(())
}

fn parse_local_time_str(ts: &str, fmt: &str) -> Result<DateTime<Local>, MySqxErr> {
    let nt = match NaiveDateTime::parse_from_str(ts, fmt) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e.to_string()));
        }
    };

    let dt = match (Local).from_local_datetime(&nt) {
        LocalResult::None => {
            return Err(MySqxErr(format!("invalid {}", ts)));
        }
        LocalResult::Single(v) => v,
        LocalResult::Ambiguous(_, _) => {
            return Err(MySqxErr(format!("invalid {}", ts)));
        }
    };

    Ok(dt)
}

#[derive(Debug)]
pub struct MySqxErr(String);

impl Display for MySqxErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MySqxErr {}

impl DatabaseError for MySqxErr {
    fn message(&self) -> &str {
        todo!()
    }

    fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
        todo!()
    }

    fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
        todo!()
    }

    fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
        todo!()
    }
}
