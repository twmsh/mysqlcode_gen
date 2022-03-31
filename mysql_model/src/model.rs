use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlArguments;
use sqlx::{Arguments, MySql, Pool, Row};
use tokio_stream::StreamExt;

use crate::mysql_util;

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
    pub async fn  get_by_id(pool: &Pool<MySql>, tz: &FixedOffset, id: i64) -> Result<Option<Self>,sqlx::Error> {
        let sql = "select * from be_user where id = ?";
        let mut rst = sqlx::query_as::<_,BeUser>(sql)
            .bind(id)
            .fetch_optional(pool).await?;

        if let Some(ref mut v) = rst {
            mysql_util::fix_read_dt_option(&mut v.last_login,tz);
            mysql_util::fix_read_dt_option(&mut v.token_expire,tz);
            mysql_util::fix_read_dt(&mut v.gmt_create,tz);
            mysql_util::fix_read_dt(&mut v.gmt_modified,tz);
        }


        Ok(rst)
    }

    pub async fn delete_by_id(pool: &Pool<MySql>,  id: i64) -> Result<u64,sqlx::Error> {
        let sql = "delete from be_user where id = ?";
        let rst = sqlx::query(sql).bind(id).execute(pool).await?;
        Ok(rst.rows_affected())
    }

    pub async fn insert(&self,pool: &Pool<MySql>, tz: &FixedOffset ) -> Result<u64,sqlx::Error> {
        let sql = "insert into be_user(name,login_name,password,salt, token,phone,email,service_flag,ref_count,last_login,token_expire,memo,gmt_create,gmt_modified) values(?,?,?,?,?,?,?,?,?,?,?,?,?,?)";
        let mut args = MySqlArguments::default();

        args.add(self.name.clone());
        args.add(self.login_name.clone());
        args.add(self.password.clone());
        args.add(self.salt.clone());

        args.add(self.token.clone());
        args.add(self.phone.clone());
        args.add(self.email.clone());
        args.add(self.service_flag.clone());
        args.add(self.ref_count.clone());

        args.add(mysql_util::fix_write_dt_option(&self.last_login,tz));
        args.add(mysql_util::fix_write_dt_option(&self.token_expire,tz));
        args.add(self.memo.clone());
        args.add(mysql_util::fix_write_dt(&self.gmt_create,tz));
        args.add(mysql_util::fix_write_dt(&self.gmt_modified,tz));

        let rst = sqlx::query_with(sql,args).execute(pool).await?;
        Ok(rst.last_insert_id())
    }

}


//-------------------------------------------

pub async fn insert_one_with_args(
    pool: &Pool<MySql>,
    db_offset: &FixedOffset,
    login_name: String,
) -> Result<u64, sqlx::Error> {
    let sql = "insert into be_user(login_name,password,salt,service_flag,gmt_create,gmt_modified,last_login) values(?,?,?,?,?,?,now())";
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

pub async fn fetch_count(pool: &Pool<MySql>, flag: i32) -> Result<u64, sqlx::Error> {
    let sql = "select count(*) from be_user where service_flag = ?";
    let row = sqlx::query(sql).bind(flag).fetch_one(pool).await?;

    let count: i64 = row.try_get(0)?;

    Ok(count as u64)
}

pub async fn del(pool: &Pool<MySql>, id: i64) -> Result<u64, sqlx::Error> {
    let sql = "delete from be_user where id = ?";

    let v = sqlx::query(sql).bind(id).execute(pool).await?;
    Ok(v.rows_affected())
}

pub async fn fetch_list_unstruct(
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

pub async fn fetch_list_page(
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

    let start_time_ex = mysql_util::fix_write_dt(&start_time, db_offset);
    let end_time_ex = mysql_util::fix_write_dt(&end_time, db_offset);

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

pub async fn fetch_one_with_args(
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

pub async fn fetch_one(
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
