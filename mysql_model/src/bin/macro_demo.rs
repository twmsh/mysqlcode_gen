use chrono::{DateTime, Local};
use mysql_codegen::MysqlEntity;

#[derive(MysqlEntity)]
#[derive(Debug)]
#[table="cf_source"]
pub struct CfSource {
    #[pk]
    #[column="id"]
    pub id: i64,
    pub name: String,

    #[column="create_time"]
    pub gmt_create: DateTime<Local>,
}

pub fn main() {
    println!("aaa");
}