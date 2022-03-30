use std::time::Duration;

use sqlx::Executor;

use sqlx::mysql::MySqlPoolOptions;

use mysql_model::model::*;
use mysql_model::mysql_util;

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
            let tz_str = tz.to_string();
            Box::pin(async move {
                let sql = format!("set time_zone = '{}'", tz_str);

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

    // let id = insert_one_with_args(&pool,&offset,"mmm".to_string()).await?;
    // println!("insert id: {}",id);

    // let count = fetch_count(&pool,100).await?;
    // println!("count: {}", count);

    // let list = fetch_list_unstruct(&pool, 100, &offset).await?;
    // println!("list: {:?}", list);

    let fmt = "%Y-%m-%d %H:%M:%S.%3f";
    let start_time = mysql_util::parse_local_time_str("2022-03-29 12:55:05.171", fmt)?;
    let end_time = mysql_util::parse_local_time_str("2022-03-29 23:59:05.171", fmt)?;
    println!("start_time: {}", start_time);
    println!("end_time: {}", end_time);
    let list = fetch_list_page(&pool, &offset, 100, 0, 3, start_time, end_time).await?;
    println!("list: {:?}", list);
    //
    // let deled = del(&pool,4).await?;
    // println!("del, affect: {}", deled);

    Ok(())
}
