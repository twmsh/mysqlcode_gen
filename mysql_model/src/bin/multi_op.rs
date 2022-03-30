use chrono::FixedOffset;
use mysql_model::model;
use mysql_model::mysql_util::{self, MySqxErr};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Executor, MySql, Pool};
use std::sync::Arc;
use std::time::{Duration, Instant};

async fn init_pool(
    db_url: &str,
    tz: &str,
    max_conn: u32,
    min_conn: u32,
) -> Result<Pool<MySql>, sqlx::Error> {
    let tz_str = tz.to_string();

    let pool = MySqlPoolOptions::new()
        .max_connections(max_conn)
        .min_connections(min_conn)
        .idle_timeout(Duration::from_secs(60 * 10))
        .after_connect(move |conn| {
            let tz_str = tz_str.clone();
            Box::pin(async move {
                let sql = format!("set time_zone = '{}'", tz_str);
                conn.execute(sql.as_str()).await?;
                println!("set end.");
                Ok(())
            })
        })
        .connect(db_url)
        .await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let tz = "+08:00";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";
    let pool = init_pool(db_url, tz, 10, 4).await?;
    let pool = Arc::new(pool);

    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let app_begin = Instant::now();

    let mut handles = Vec::new();

    for i in 0..10 {
        let pool_cl = pool.clone();
        let offset_cl = offset;
        let num = i;
        let handle = tokio::spawn(async move {
            let begin_t = Instant::now();
            let v = model::fetch_count(&pool_cl, 100).await;
            println!(
                "a, {}, use:{}, result: {:?}",
                num,
                begin_t.elapsed().as_millis(),
                v
            );
        });
        handles.push(handle);
    }

    for i in 0..10 {
        let pool_cl = pool.clone();
        let offset_cl = offset;
        let num = i;
        let handle = tokio::spawn(async move {
            let begin_t = Instant::now();
            let v = model::fetch_list_unstruct(&pool_cl, 100, &offset_cl).await;
            println!(
                "b, {}, use:{}, result: {:?}",
                num,
                begin_t.elapsed().as_millis(),
                v
            );
        });
        handles.push(handle);
    }

    for h in handles {
        let _= h.await;
    }
    println!("app end. use:{}", app_begin.elapsed().as_millis());
    Ok(())
}

async fn main2() -> Result<(), sqlx::Error> {
    let tz = "+08:00";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";
    let pool = init_pool(db_url, tz, 20, 4).await?;
    let pool = Arc::new(pool);

    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let pool_a = pool.clone();
    let offset_a = offset;
    let handle_a = tokio::spawn(async move {
        for i in 0..10 {
            let begin_t = Instant::now();
            let v = model::fetch_count(&pool_a, 100).await;
            println!(
                "a, {}, use:{}, result: {:?}",
                i,
                begin_t.elapsed().as_millis(),
                v
            );
        }
    });

    let pool_b = pool.clone();
    let offset_b = offset;
    let handle_b = tokio::spawn(async move {
        for i in 0..10 {
            let begin_t = Instant::now();
            let v = model::fetch_list_unstruct(&pool_b, 100, &offset_b).await;
            println!(
                "b, {}, use:{}, result: {:?}",
                i,
                begin_t.elapsed().as_millis(),
                v
            );
        }
    });

    handle_a.await;
    handle_b.await;
    println!("app end.");
    Ok(())
}
