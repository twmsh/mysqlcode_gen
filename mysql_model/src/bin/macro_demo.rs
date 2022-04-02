use chrono::{DateTime, Local};
use mysql_codegen::MysqlEntity;

#[derive(MysqlEntity)]
pub struct Entity {
    pub id: i64,
    pub name: String,
    pub gmt_create: DateTime<Local>,
}

pub fn main() {

}