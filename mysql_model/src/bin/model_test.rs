use std::sync::Arc;
use chrono::{DateTime, Local};
use mysql_model::model::BeUser;
use mysql_model::mysql_util;
use mysql_model::mysql_util::MySqxErr;

#[tokio::main]
async fn main()-> Result<(),sqlx::Error> {
    let tz = "+08:00";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";
    let pool = mysql_util::init_pool(db_url, tz, 20, 4).await?;
    let pool = Arc::new(pool);

    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

    let now = Local::now();

    let mut obj = BeUser {
        id: 0,
        name: Some("tom".to_string()),
        login_name: "tom".to_string(),
        password: "tom".to_string(),
        salt: "1111".to_string(),
        token: Some("token".to_string()),
        phone: Some("phone".to_string()),
        email: Some("email".to_string()),
        service_flag: Some(200),
        ref_count: Some(10),
        last_login: Some(now),
        token_expire: Some(now),
        memo: Some("memo".to_string()),
        gmt_create: now,
        gmt_modified: now,
    };

    let rst = obj.insert(&pool,&offset).await?;
    println!("insert id: {}",rst);

    obj.id = rst as i32;

    let rst = BeUser::get_by_id(&pool,&offset,obj.id as i64).await?;
    println!("select: {:?}",rst);

    let rst = BeUser::delete_by_id(&pool,obj.id as i64).await?;
    println!("del rows: {:?}",rst);
    Ok(())
}