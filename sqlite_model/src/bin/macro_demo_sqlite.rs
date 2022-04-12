#![allow(dead_code, unused_imports)]

use chrono::{DateTime, Local};
// use serde::{Serialize,Deserialize};
// use sqlite_model::sqlite_util::MySqxErr;

use sqlite_codegen::SqliteEntity;
// use sqlite_model::sqlite_util;
use sqlx::{Arguments, FromRow};
use sqlite_model::sqlite_util;
use sqlite_model::cf_model::BeUser;
use sqlite_model::sqlite_util::MySqxErr;

//
// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//
//     let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";
//     let pool = mysql_util::init_pool(db_url, tz, 20, 4).await?;
//
//     let offset = match mysql_util::parse_timezone(tz) {
//         Ok(v) => v,
//         Err(e) => {
//             return Err(MySqxErr(e).into());
//         }
//     };
//
//     let beuser = BeUser::load(1, &pool, &offset).await?;
//     println!("beuser: {:?}", beuser);
//
//     let affect = BeUser::delete(10, &pool).await?;
//     println!("affect: {:?}", affect);
//
//     let now = Local::now();
//     let login_name = now.timestamp().to_string();
//     let mut beuser = BeUser {
//         id: 0,
//         name: Some("name".to_string()),
//         login_name,
//         password: "password".to_string(),
//         salt: "salt".to_string(),
//         token: Some("token".to_string()),
//         phone: Some("phone".to_string()),
//         email: Some("email".to_string()),
//         service_flag: Some(20),
//         ref_count: Some(10),
//         last_login: Some(now),
//         token_expire: Some(now),
//         memo: Some("memo".to_string()),
//         gmt_create: now,
//         gmt_modified: now,
//     };
//
//     let affect = beuser.insert(&pool, &offset).await?;
//     println!("insert: {:?}", affect);
//
//     beuser.id = affect as i32;
//
//     let now = Local::now();
//     beuser.gmt_modified = now;
//     beuser.ref_count = Some(1234);
//
//     let affect = beuser.update(&pool, &offset).await?;
//     println!("update: {:?}", affect);
//
//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {


    println!("aaa");
    let tz = "+08:00";
    let db_url = r#"sqlite:C:\Users\tom\develop\RustProjects\mysql_codegen\doc\a.db"#;
    let pool = sqlite_util::init_sqlite_pool(db_url,4,2).await?;

    let offset = match sqlite_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let beuser = BeUser::load(1, &pool, &offset).await?;
    println!("beuser: {:?}", beuser);

    let affect = BeUser::delete(10, &pool).await?;
    println!("affect: {:?}", affect);

    let now = Local::now();
    let login_name = now.timestamp().to_string();
    let mut beuser = BeUser {
        id: 0,
        name: Some("name".to_string()),
        login_name,
        password: "password".to_string(),
        salt: "salt".to_string(),
        token: Some("token".to_string()),
        phone: Some("phone".to_string()),
        email: Some("email".to_string()),
        service_flag: Some(20),
        ref_count: Some(10),
        last_login: Some(now),
        token_expire: Some(now),
        memo: Some("memo".to_string()),
        gmt_create: now,
        gmt_modified: now,
    };

    let affect = beuser.insert(&pool, &offset).await?;
    println!("insert: {:?}", affect);

    beuser.id = affect as i64;

    let now = Local::now();
    beuser.gmt_modified = now;
    beuser.ref_count = Some(1234);

    let affect = beuser.update(&pool, &offset).await?;
    println!("update: {:?}", affect);

    Ok(())
}