use chrono::{DateTime, Local};
// use serde::{Serialize,Deserialize};
use mysql_model::mysql_util::MySqxErr;

use mysql_codegen::MysqlEntity;
use mysql_model::mysql_util;
use sqlx::{Arguments,FromRow};


#[derive(FromRow,Debug,Clone)]
#[derive(MysqlEntity)]
#[table="be_user"]
pub struct BeUser {
    #[pk]
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("aaa");
    let tz = "+08:00";
    let db_url = "mysql://cf_user:cf123456@192.168.1.26:3306/cf_rs";
    let pool = mysql_util::init_pool(db_url, tz, 20, 4).await?;

    let offset = match mysql_util::parse_timezone(tz) {
        Ok(v) => v,
        Err(e) => {
            return Err(MySqxErr(e).into());
        }
    };

   let beuser = BeUser::get_by_id(&pool,&offset,1).await?;
    println!("beuser: {:?}",beuser);

    let affect = BeUser::delete_by_id(&pool,10).await?;
    println!("affect: {:?}",affect);


    Ok(())
}