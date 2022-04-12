use chrono::{DateTime, Local};
use sqlite_codegen::SqliteEntity;
use serde::{Deserialize, Serialize};
use sqlx::Arguments;

use crate::sqlite_util;

#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_user"]
pub struct BeUser {
    #[pk]
    pub id: i64,

    pub name: Option<String>,

    pub login_name: String,

    pub password: String,

    pub salt: String,

    pub token: Option<String>,

    pub phone: Option<String>,

    pub email: Option<String>,

    pub service_flag: Option<i32>,

    pub ref_count: Option<i32>,

    pub last_login: Option<DateTime<Local>>,

    pub token_expire: Option<DateTime<Local>>,

    pub memo: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_dfnode"]
pub struct CfDfnode {
    #[pk]
    pub id: i64,

    pub node_sid: String,

    pub name: String,

    pub ip: String,

    pub url: String,

    pub node_type: i32,

    pub sort_num: Option<i32>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_dfdb"]
pub struct CfDfdb {
    #[pk]
    pub id: i64,

    pub db_sid: String,

    pub name: String,

    pub node_sid: String,

    pub capacity: i64,

    pub auto_match: i32,

    pub bw_flag: i32,

    pub fp_flag: i32,

    pub sort_num: Option<i32>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_dfsource"]
pub struct CfDfsource {
    #[pk]
    pub id: i64,

    pub src_sid: String,

    pub name: String,

    pub node_sid: String,

    pub src_url: String,

    pub push_url: String,

    pub ip: String,

    pub src_state: i32,

    pub src_config: String,

    pub grab_type: i32,

    pub io_flag: i32,

    pub direction: i32,

    pub tp_id: Option<String>,

    pub upload_flag: i32,

    pub location_name: Option<String>,

    pub resolution_ratio: Option<String>,

    pub coordinate: Option<String>,

    pub sort_num: i32,

    pub trip_line: i64,

    pub rtcp_utc: i32,

    pub lane_desc: Option<String>,

    pub lane_count: i32,

    pub memo: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_poi"]
pub struct CfPoi {
    #[pk]
    pub id: i64,

    pub poi_sid: String,

    pub db_sid: String,

    pub name: String,

    pub gender: Option<i32>,

    pub identity_card: Option<String>,

    pub threshold: i32,

    pub tp_id: Option<String>,

    pub feature_ids: String,

    pub cover: Option<i32>,

    pub tag: Option<String>,

    pub imp_tag: Option<String>,

    pub memo: Option<String>,

    pub flag: Option<i32>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_facetrack"]
pub struct CfFacetrack {
    #[pk]
    pub id: i64,

    pub ft_sid: String,

    pub src_sid: String,

    pub img_ids: String,

    pub matched: Option<i32>,

    pub judged: Option<i32>,

    pub alarmed: Option<i32>,

    pub most_person: Option<String>,

    pub most_score: Option<f64>,

    pub gender: Option<i32>,

    pub age: Option<i32>,

    pub glasses: Option<i32>,

    pub direction: Option<i32>,

    pub plane_score: Option<f64>,

    pub mask: Option<i32>,

    pub moustache: Option<i32>,

    pub hat: Option<i32>,

    pub tag: Option<String>,

    pub flag: i32,

    pub db_flag: Option<i32>,

    pub db_sid: Option<String>,

    pub feature_ids: Option<String>,

    pub obj_id: Option<String>,

    pub submit_id: Option<String>,

    pub submit_time: Option<DateTime<Local>>,

    pub capture_time: DateTime<Local>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_dictory"]
pub struct CfDictory {
    #[pk]
    pub id: i64,

    pub group_label: String,

    pub group_key: String,

    pub item_label: String,

    pub item_key: String,

    pub item_value: Option<String>,

    pub sort_num: Option<i32>,

    pub memo: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_delpoi"]
pub struct CfDelpoi {
    #[pk]
    pub id: i64,

    pub poi_id: i64,

    pub poi_sid: String,

    pub db_sid: String,

    pub name: String,

    pub tp_id: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_cartrack"]
pub struct CfCartrack {
    #[pk]
    pub id: i64,

    pub sid: String,

    pub src_sid: String,

    pub img_ids: String,

    pub alarmed: i32,

    pub most_coi: Option<String>,

    pub plate_judged: i32,

    pub vehicle_judged: i32,

    pub move_direct: i32,

    pub car_direct: Option<String>,

    pub plate_content: Option<String>,

    pub plate_confidence: Option<f64>,

    pub plate_type: Option<String>,

    pub car_color: Option<String>,

    pub car_brand: Option<String>,

    pub car_top_series: Option<String>,

    pub car_series: Option<String>,

    pub car_top_type: Option<String>,

    pub car_mid_type: Option<String>,

    pub tag: Option<String>,

    pub flag: i32,

    pub obj_id: Option<String>,

    pub submit_id: Option<String>,

    pub submit_time: Option<DateTime<Local>>,

    pub is_realtime: i32,

    pub capture_time: DateTime<Local>,

    pub capture_ts: i64,

    pub capture_pts: i64,

    pub lane_num: i32,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_coi"]
pub struct CfCoi {
    #[pk]
    pub id: i64,

    pub sid: String,

    pub group_sid: String,

    pub plate_content: String,

    pub plate_type: Option<String>,

    pub car_brand: Option<String>,

    pub car_series: Option<String>,

    pub car_size: Option<String>,

    pub car_type: Option<String>,

    pub owner_name: Option<String>,

    pub owner_idcard: Option<String>,

    pub owner_phone: Option<String>,

    pub owner_address: Option<String>,

    pub flag: i32,

    pub tag: Option<String>,

    pub imp_tag: Option<String>,

    pub memo: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_coi_group"]
pub struct CfCoiGroup {
    #[pk]
    pub id: i64,

    pub sid: String,

    pub name: String,

    pub bw_flag: i32,

    pub memo: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_gate"]
pub struct CfGate {
    #[pk]
    pub id: i64,

    pub src_sid: String,

    pub sid: String,

    pub name: String,

    pub uni_code: Option<String>,

    pub flag: Option<i32>,

    pub ac_config: String,

    pub ac_type: Option<i32>,

    pub sort_num: Option<i32>,

    pub memo: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
#[derive(sqlx::FromRow, SqliteEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_gatehistory"]
pub struct CfGatehistory {
    #[pk]
    pub id: i64,

    pub ft_sid: String,

    pub src_sid: String,

    pub src_name: String,

    pub gate_sid: String,

    pub gate_name: String,

    pub poi_sid: String,

    pub poi_name: String,

    pub poi_idcard: Option<String>,

    pub gmt_create: DateTime<Local>,

    pub gmt_modified: DateTime<Local>,
}
