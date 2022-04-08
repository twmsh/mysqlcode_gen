use chrono::{DateTime, Local};
use mysql_codegen::MysqlEntity;
use serde::{Deserialize, Serialize};
use sqlx::Arguments;

use crate::mysql_util;

/* 用来定义某个模块可以进行哪些操作，每个操作对应一个url调用 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_function"]
pub struct BeFunction {
    #[pk]
    pub id: i64,

    /* 操作名称 */
    pub name: String,

    /* 对应的模块id */
    pub module_id: i64,

    /* 对应的url, 相对路径 */
    pub url: String,

    /* 是否是该模块的默认操作 */
    pub is_default: i32,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 模块表，可以用来做后台导航菜单 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_module"]
pub struct BeModule {
    /* id */
    #[pk]
    pub id: i64,

    /* 模块名称 */
    pub name: String,

    /* 父模块id */
    pub parent_id: i64,

    /* 是否是叶子节点(不含子模块) */
    pub is_leaf: i32,

    /* 是否显示 */
    pub is_display: i32,

    /* 图标 */
    pub icon: Option<String>,

    /* 排序，升序 */
    pub sort_num: i32,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 后台操作日志 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_oplog"]
pub struct BeOplog {
    /* id */
    #[pk]
    pub id: i64,

    /* 后台用户 */
    pub user: String,

    /* 访问的url */
    pub http_url: String,

    /* 访问的ip */
    pub http_ip: String,

    /* 浏览器ip */
    pub http_ua: Option<String>,

    /* 0:GET 1:POST */
    pub http_method: i32,

    /* get parameter/ post data */
    pub http_data: Option<String>,

    /* be_function url */
    pub function_id: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_role"]
pub struct BeRole {
    /* id */
    #[pk]
    pub id: i64,

    /* 角色名称 */
    pub name: String,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_role_function"]
pub struct BeRoleFunction {
    /* id */
    #[pk]
    pub id: i64,

    /* 角色id */
    pub role_id: i64,

    /* 操作id */
    pub function_id: i64,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_user"]
pub struct BeUser {
    /* id */
    #[pk]
    pub id: i64,

    /* 昵称 */
    pub name: Option<String>,

    /* 登录名 */
    pub login_name: String,

    /* 密码的md5 */
    pub password: String,

    /* md5的salt */
    pub salt: String,

    /* 用户状态 */
    pub service_flag: Option<i32>,

    pub last_login: Option<DateTime<Local>>,

    /* token */
    pub token: Option<String>,

    /* token 失效时间 */
    pub token_expire: Option<DateTime<Local>>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "be_user_role"]
pub struct BeUserRole {
    /* id */
    #[pk]
    pub id: i64,

    pub user_id: i64,

    pub role_id: i64,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 存放监控报警规则 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_alarm_config"]
pub struct CfAlarmConfig {
    #[pk]
    pub id: i64,

    /* 规则名称 */
    pub name: String,

    /* 视频源id */
    pub src_id: String,

    /* 监控目标分组id */
    pub poigroup_id: i64,

    /* 1:白名单报警 2：黑名单报警 */
    pub alarm_type: i32,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 存放监控报警历史记录 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_alarm_history"]
pub struct CfAlarmHistory {
    #[pk]
    pub id: i64,

    /* 视频源id */
    pub src_id: String,

    /* facetrack id */
    pub facetrack_id: String,

    /* person id */
    pub person_id: Option<String>,

    /* 由哪个报警规则触发的 */
    pub alarm_config_id: i64,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 监控摄像头表 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_camera"]
pub struct CfCamera {
    #[pk]
    pub id: i64,

    /* 对应api系统中的uuid */
    pub src_id: String,

    /* 摄像头名称，对应api系统中的description */
    pub name: String,

    /* 摄像头分组id */
    pub category_id: i64,

    /* 0:暂停状态 1：启动状态 */
    pub flag: i32,

    /* 统一编码GB/28181 */
    pub uni_code: Option<String>,

    /* 本地编码 */
    pub local_code: Option<String>,

    /* 摄像头安装地址 */
    pub install_address: Option<String>,

    /* 坐标 */
    pub coordinate: Option<String>,

    pub ip_address: Option<String>,

    /* 摄像头类型 0：其它 1:海康 2：大华 3：宇视  */
    pub model_type: i32,

    /* 摄像头型号名称 */
    pub model_name: Option<String>,

    pub camera_username: Option<String>,

    pub camera_password: Option<String>,

    pub play_url: String,

    pub cjd_url: String,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,

    pub debug_url: Option<String>,

    pub cjd_uuid: String,

    pub cjd_subid: i32,

    pub io_flag: Option<i32>,

    pub screen_no: i32,
}

/* 摄像头分组 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_camera_category"]
pub struct CfCameraCategory {
    #[pk]
    pub id: i64,

    /* 名称 */
    pub name: String,

    /* 父节点id */
    pub parent_id: i64,

    /* 是否是叶子节点 1：是 0：否 */
    pub is_leaf: i32,

    /* 排序号 */
    pub sort_num: i32,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 摄像头采集参数配置 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_camera_config"]
pub struct CfCameraConfig {
    #[pk]
    pub id: i64,

    /* 对应api系统中的uuid */
    pub src_id: String,

    /* 配置json */
    pub config: String,

    /* 默认配置json */
    pub default_config: String,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_dictory"]
pub struct CfDictory {
    #[pk]
    pub id: i64,

    /* 类似于索引键 */
    pub dic_key: String,

    /* 值 */
    pub dic_value: String,

    /* 存放类似于value的值 */
    pub dic_name: String,

    /* 归属类别 */
    pub dic_tag: String,

    /* 排序 */
    pub sort_no: i32,

    /* 保留字段 */
    pub flag: i32,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 后台进程调用getunprocessedmatchedfacetrack，获得facetrack插入该表 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_facetrack_history"]
pub struct CfFacetrackHistory {
    #[pk]
    pub id: i64,

    /* facetrack id */
    pub facetrack_id: String,

    /* 视频源id */
    pub src_id: String,

    /* 自动匹配的Transaction标识 */
    pub transaction_id: String,

    /* 根据阈值裁定出来person */
    pub judge_person: Option<String>,

    /* 裁定person时的分数, 从percent或score中来 */
    pub judge_score: Option<f64>,

    /* 裁定person时的分数,原始值 */
    pub judge_score_orig: Option<f64>,

    /* 裁定person时的百分比(阈值) */
    pub judge_percent: Option<f64>,

    /* 匹配分数最高的人（不一定就是裁定出来的人） */
    pub most_person: Option<String>,

    /* 最高的匹配分数 */
    pub most_score: Option<f64>,

    /* 最高的匹配分数,原始值,从percent或score中来 */
    pub most_score_orig: Option<f64>,

    /* 最高的匹配百分比 */
    pub most_percent: Option<f64>,

    pub imgs: String,

    /* 附带说明 */
    pub descriptor: String,

    /* 根据阈值是否判定到某个人 0:否 1:是 */
    pub judged: i32,

    /* 是否报警 0：否 1：是 */
    pub alarmed: i32,

    /* 对应state字段，0:未处理，1：已添加到某个person */
    pub bind_state: i32,

    /* 对应api中的createdate,facetrack采集时间 */
    pub capture_time: DateTime<Local>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,

    pub flag: i32,

    /* 是否有带眼镜，默认为0，1 戴眼镜，2没带眼镜 */
    pub glasses: i32,

    pub age: i32,

    /* 是否有胡子。0：未知 ，1：有 ，2没有 */
    pub moustache: i32,

    /* 是否带帽子。0：未知 ，1：有 ，2没有 */
    pub hat: i32,

    /* 0：女，1：男 */
    pub gender: i32,
}

/* person对象所关联的facetrack */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_person_ft"]
pub struct CfPersonFt {
    #[pk]
    pub id: i64,

    /* person id(guid) */
    pub person_id: String,

    /* facetrack id */
    pub facetrack_id: String,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 监控目标人物表 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_poi"]
pub struct CfPoi {
    #[pk]
    pub id: i64,

    /* 对应api系统中的UUID */
    pub person_id: String,

    /* 名称 */
    pub name: String,

    /* -1:未知 0:女 1: 男  */
    pub gender: i32,

    /* 身份证号码 */
    pub identity_card: Option<String>,

    /* 报警阀值,1-100之间整数 */
    pub alarm_threshold: i32,

    /* 保留字段，暂未使用 */
    pub flag: i32,

    /* 导入标签 */
    pub imp_tag: Option<String>,

    /* 人物头像列表（,分隔开） */
    pub imgs: Option<String>,

    pub upload_imgs: Option<String>,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,

    pub resident_flag: Option<i32>,
}

/* 监控目标分组 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_poigroup"]
pub struct CfPoigroup {
    #[pk]
    pub id: i64,

    /* 名称 */
    pub name: String,

    /* 0: 非默认分组 1：默认分组 */
    pub is_default: i32,

    /* 备注 */
    pub memo: Option<String>,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

/* 监控目标分组关联表，存放分组成员 */
#[derive(sqlx::FromRow, MysqlEntity, Serialize, Deserialize, Debug, Clone)]
#[table = "cf_poigroup_map"]
pub struct CfPoigroupMap {
    #[pk]
    pub id: i64,

    /* 分组id */
    pub group_id: i64,

    /* poi id */
    pub poi_id: i64,

    /* 创建时间 */
    pub gmt_create: DateTime<Local>,

    /* 修改时间 */
    pub gmt_modified: DateTime<Local>,
}

