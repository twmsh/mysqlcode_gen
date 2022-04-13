#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]

use std::env;
// use chrono::FixedOffset;
use sqlite_model::model;
use sqlite_model::sqlite_util::{self, MySqxErr};
// use sqlx::mysql::MySqlPoolOptions;
// use sqlx::{Executor, MySql, Pool};
use chrono::Local;
use sqlite_model::cf_model::BeUser;
use std::sync::Arc;
use std::time::Instant;

use log::debug;
use tokio::sync::Barrier;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut db_file = r#"C:\Users\tom\develop\RustProjects\mysql_codegen\doc\a.db"#.to_string();
    let mut count = 10;

    env_logger::Builder::new()
        .format(|buf, record| {
            let ts = buf.timestamp_millis();

            writeln!("[{}] {}", ts, record.args())
        })
        .init();

    let mut args = env::args();
    if args.len() == 3 {
        println!("{:?}", args);
        count = args.nth(1).unwrap().parse().unwrap();
        db_file = args.nth(0).unwrap().clone();
        println!("{}, {}", count, db_file);
    }

    let barrier_count = count * 3;
    let barrier = Arc::new(Barrier::new(barrier_count));

    let tz = "+08:00";
    let db_url = format!(r#"sqlite:{}"#, db_file);
    let pool = sqlite_util::init_sqlite_pool(db_url.as_str(), 100, 4).await?;
    let pool = Arc::new(pool);

    let offset = match sqlite_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let app_begin = Instant::now();

    let mut handles = Vec::new();

    for i in 0..count {
        let pool_cl = pool.clone();
        let offset_cl = offset;
        let num = i;
        let barrier_cl = barrier.clone();
        let handle = tokio::spawn(async move {
            barrier_cl.wait();

            let begin_t = Instant::now();
            let v = model::fetch_count(&pool_cl, 20).await;
            debug!(
                "a[{}], use:{}, result: {:?}",
                num,
                begin_t.elapsed().as_millis(),
                v
            );

            if let Err(e) = v {
                panic!("a[{}], err: {:?}", num, e);
            }
        });
        handles.push(handle);
    }

    for i in 0..count {
        let pool_cl = pool.clone();
        let offset_cl = offset;
        let num = i;
        let barrier_cl = barrier.clone();
        let handle = tokio::spawn(async move {
            let now = Local::now();
            let login_name = now.timestamp_nanos().to_string();

            // println!(
            //     "c[{}], login_name: {}",num,login_name);

            let beuser = BeUser {
                id: 0,
                name: Some("name".to_string()),
                login_name,
                password: "password".to_string(),
                salt: "salt".to_string(),
                token: Some("token".to_string()),
                phone: Some("中文".to_string()),
                email: Some("email".to_string()),
                service_flag: Some(20),
                ref_count: Some(10),
                last_login: Some(now),
                token_expire: Some(now),
                memo: Some("memo".to_string()),
                gmt_create: now,
                gmt_modified: now,
            };

            barrier_cl.wait();

            let begin_t = Instant::now();
            let v = beuser.insert(&pool_cl, &offset_cl).await;

            debug!(
                "c[{}], use:{}, result: {:?}",
                num,
                begin_t.elapsed().as_millis(),
                v
            );
            if let Err(e) = v {
                panic!("c[{}], err: {:?}", num, e);
            }
        });
        handles.push(handle);
    }

    for i in 0..count {
        let pool_cl = pool.clone();
        let offset_cl = offset;
        let num = i;
        let barrier_cl = barrier.clone();
        let handle = tokio::spawn(async move {
            barrier_cl.wait();

            let begin_t = Instant::now();
            let v = model::fetch_list_unstruct(&pool_cl, 20, &offset_cl).await;
            if let Err(e) = v {
                panic!("b[{}], err: {:?}", num, e);
            }
            debug!(
                "b[{}], use:{}, result: {:?}",
                num,
                begin_t.elapsed().as_millis(),
                v.unwrap().len()
            );
        });
        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }
    debug!("app end. use:{}", app_begin.elapsed().as_millis());
    Ok(())
}

/*
async fn main2() -> Result<(), sqlx::Error> {
    let tz = "+08:00";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";
    let pool = sqlite_util::init_mysql_pool(db_url, tz, 20, 4).await?;
    let pool = Arc::new(pool);

    let offset = match sqlite_util::parse_timezone(tz) {
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
*/
