use chrono::{DateTime, Local};
use mysql_codegen::MysqlEntity;

#[derive(MysqlEntity)]
#[table="cf_source"]
pub struct CfSource {
    #[pk]
    #[column="id"]
    pub id: i64,
    pub name: String,

    #[column="login_time"]
    pub last_login: std::option::Option<chrono::DateTime<chrono::Local>>,

}

pub fn main() {
    println!("aaa");
}